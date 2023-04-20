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

type ThemeAdapter struct {
	mutex           *sync.Mutex
	config          *entities.Config
	globalVariables map[string]any
	knownTemplates  map[string]time.Time
}

var _ ports.ThemePort = &ThemeAdapter{}

func init() {
	pongo2.RegisterFilter("truncate", func(in *pongo2.Value, param *pongo2.Value) (*pongo2.Value, *pongo2.Error) {
		if (in.String() != "") && (param.Integer() > 0) && (param.Integer() < len(in.String())) {
			return pongo2.AsValue(in.String()[:param.Integer()]), nil
		}
		return in, nil
	})
}

func NewThemeAdapter(config *entities.Config) *ThemeAdapter {
	return &ThemeAdapter{
		mutex:           &sync.Mutex{},
		config:          config,
		globalVariables: make(map[string]any),
		knownTemplates:  make(map[string]time.Time),
	}
}

func (a *ThemeAdapter) getPongoContext(site *entities.Site) pongo2.Context {
	ctx := pongo2.Context{
		"site": site,
		"bytes_to_string": func(b []byte) string {
			return string(b)
		},
		"get_path": func(u *url.URL) string {
			return u.String()
		},
		"get_subpages": func(page *entities.Page) []*entities.Page {
			subpages := []*entities.Page{}
			for _, p := range site.Pages {
				pagePath := strings.TrimPrefix(p.Path.Path, site.BaseURL)
				if !strings.HasPrefix(pagePath, "/") {
					pagePath = "/" + pagePath
				}
				parentPagePath := strings.TrimPrefix(page.Path.Path, site.BaseURL)
				if !strings.HasPrefix(parentPagePath, "/") {
					parentPagePath = "/" + parentPagePath
				}
				if !strings.HasPrefix(pagePath, parentPagePath) {
					continue
				}
				pPath := strings.TrimPrefix(pagePath, parentPagePath)

				if pPath != "" && strings.Count(pPath, "/") == 1 && strings.HasPrefix(pPath, "/") {
					subpages = append(subpages, p)
				}
			}
			return subpages
		},
		"get_rootpages": func() []*entities.Page {
			rootpages := []*entities.Page{}
			for _, p := range site.Pages {
				path := strings.TrimPrefix(p.Path.Path, site.BaseURL)
				if !strings.HasPrefix(path, "/") {
					path = "/" + path
				}
				if strings.Count(path, "/") == 1 {
					rootpages = append(rootpages, p)
				}
			}
			return rootpages
		},
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

func (a *ThemeAdapter) AddTemplateVariable(name string, value any) {
	a.globalVariables[name] = value
}

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

func (a *ThemeAdapter) touchTemplate(templatePath string) {
	a.mutex.Lock()
	if _, ok := a.knownTemplates[templatePath]; !ok {
		if info, err := os.Stat(templatePath); err == nil {
			a.knownTemplates[templatePath] = info.ModTime()
		}
	}
	a.mutex.Unlock()
}
