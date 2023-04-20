package adapters

import (
	"fmt"
	"io"
	"net/url"
	"os"
	"strings"
	"sync"
	"time"

	"github.com/flosch/pongo2/v6"
	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
)

// ThemeAdapter is the adapter for the theme port.
// It is used to render the pages.
type ThemeAdapter struct {
	mutex           *sync.Mutex
	config          *entities.Config
	globalVariables map[string]any
	knownTemplates  map[string]time.Time
}

var _ ports.ThemePort = &ThemeAdapter{}

// init registers the filters for pongo2
func init() {
	pongo2.RegisterFilter("truncate", func(in *pongo2.Value, param *pongo2.Value) (*pongo2.Value, *pongo2.Error) {
		if (in.String() != "") && (param.Integer() > 0) && (param.Integer() < len(in.String())) {
			return pongo2.AsValue(in.String()[:param.Integer()]), nil
		}
		return in, nil
	})
}

// NewThemeAdapter returns a new ThemeAdapter
func NewThemeAdapter(config *entities.Config) *ThemeAdapter {
	return &ThemeAdapter{
		mutex:           &sync.Mutex{},
		config:          config,
		globalVariables: make(map[string]any),
		knownTemplates:  make(map[string]time.Time),
	}
}

// getPongoContext returns a pongo2 context with the site and some helper functions
func (a *ThemeAdapter) getPongoContext(site *entities.Site) pongo2.Context {
	ctx := pongo2.Context{
		"site": site,
		// bytes_to_string converts a byte slice to a string
		"bytes_to_string": func(b []byte) string {
			return string(b)
		},
		// get_path returns the path of a page
		// if a base URL is given, the base URL is prepended
		"get_path": func(u *url.URL, baseURL ...string) string {
			if len(baseURL) > 0 {
				uri := baseURL[0] + u.Path
				return strings.ReplaceAll(uri, "//", "/")
			}
			return u.String()
		},
		// get_subpages returns the subpages of a page
		"get_subpages": func(page *entities.Page) []*entities.Page {
			return page.Children
		},
		// get_rootpages returns the root pages. The root pages are the pages without a parent.
		"get_rootpages": func() []*entities.Page {
			for _, page := range site.Pages {
				if page.Parent == nil {
					return page.Children
				}
			}
			return []*entities.Page{}
		},
		// is_active returns true if the given page path is the current page path or a parent of the current page path.
		"is_active": func(pagePath string, exact bool) bool {
			if exact {
				return site.Path.Path == pagePath
			}
			return strings.HasPrefix(site.Path.Path, pagePath)
		},
	}
	for k, v := range a.globalVariables {
		ctx[k] = v
	}
	return ctx
}

// AddTemplateVariable adds a variable to the global context.
func (a *ThemeAdapter) AddTemplateVariable(name string, value any) {
	a.globalVariables[name] = value
}

// TemplateRenderer returns a renderer for a page.
func (a *ThemeAdapter) TemplateRenderer() ports.ThemeRenderer {
	return ports.ThemeRenderer(func(site *entities.Site, w io.Writer) error {
		if site.Page == nil {
			return fmt.Errorf("no page to render")
		}
		templatePath := site.Page.Template.String()
		if templatePath == "" {
			return fmt.Errorf("no template to render")
		}

		a.touchTemplate(templatePath)

		tpl, err := pongo2.FromCache(templatePath)
		if err != nil {
			return fmt.Errorf("could not load template: %w", err)
		}
		out, err := tpl.ExecuteBytes(a.getPongoContext(site))
		if err != nil {
			return fmt.Errorf("could not execute template: %w", err)
		}
		w.Write(out)
		return nil
	})
}

// NotFoundRenderer returns a renderer for the 404 page.
func (a *ThemeAdapter) NotFoundRenderer() ports.ThemeRenderer {
	return ports.ThemeRenderer(func(site *entities.Site, w io.Writer) error {

		templatePath, err := a.config.ThemePath.Join("templates", "404.html")
		if err != nil {
			return nil
		}

		a.touchTemplate(templatePath.String())

		tpl, err := pongo2.FromCache(templatePath.String())
		if err != nil {
			return fmt.Errorf("could not load template: %w", err)
		}
		out, err := tpl.ExecuteBytes(a.getPongoContext(site))
		if err != nil {
			return fmt.Errorf("could not execute template: %w", err)
		}
		w.Write(out)
		return nil
	})
}

// WatchTemplates checks for changes in the templates and reloads them if necessary.
func (a *ThemeAdapter) WatchTemplates() {

	ticker := time.NewTicker(5 * time.Second)
	for {
		select {
		case <-ticker.C:
			a.mutex.Lock()
			for templatePath, lastModTime := range a.knownTemplates {
				if info, err := os.Stat(templatePath); err == nil {
					if info.ModTime().After(lastModTime) {
						pongo2.DefaultSet.CleanCache(templatePath)
						a.knownTemplates[templatePath] = info.ModTime()
					}
				}
			}
			a.mutex.Unlock()
		}
	}
}

// touchTemplate adds a template to the known templates map and sets the last modification time.
func (a *ThemeAdapter) touchTemplate(templatePath string) {
	a.mutex.Lock()
	if _, ok := a.knownTemplates[templatePath]; !ok {
		if info, err := os.Stat(templatePath); err == nil {
			a.knownTemplates[templatePath] = info.ModTime()
		}
	}
	a.mutex.Unlock()
}
