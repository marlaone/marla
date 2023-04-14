package adapters

import (
	"fmt"
	"io"
	"net/url"
	"path/filepath"
	"strings"

	"github.com/flosch/pongo2/v6"
	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
)

type ThemeAdapter struct {
	config          *entities.Config
	globalVariables map[string]any
}

var _ ports.ThemePort = &ThemeAdapter{}

func init() {
	pongo2.RegisterFilter("truncate", func(in *pongo2.Value, param *pongo2.Value) (*pongo2.Value, *pongo2.Error) {
		return pongo2.AsValue(in.String()[:param.Integer()]), nil
	})
}

func NewThemeAdapter(config *entities.Config) *ThemeAdapter {
	return &ThemeAdapter{
		config:          config,
		globalVariables: make(map[string]any),
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
				if !strings.HasPrefix(p.Path.Path, page.Path.Path) {
					continue
				}
				pPath := strings.TrimPrefix(p.Path.Path, page.Path.Path)
				if pPath != "" && strings.Count(pPath, "/") == 1 {
					subpages = append(subpages, p)
				}
			}
			return subpages
		},
		"get_rootpages": func() []*entities.Page {
			rootpages := []*entities.Page{}
			for _, p := range site.Pages {
				if strings.Count(p.Path.Path, "/") == 1 {
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

func (a *ThemeAdapter) IndexRenderer() ports.ThemeRenderer {
	return ports.ThemeRenderer(func(site *entities.Site, w io.Writer) error {
		tpl, err := pongo2.FromFile(filepath.Join(a.config.ThemePath.String(), "templates", "index.html"))
		if err != nil {
			return fmt.Errorf("could not load index template: %w", err)
		}
		out, err := tpl.ExecuteBytes(a.getPongoContext(site))
		if err != nil {
			return fmt.Errorf("could not execute index template: %w", err)
		}
		w.Write(out)
		return nil
	})
}

func (a *ThemeAdapter) PageRenderer() ports.ThemeRenderer {
	return ports.ThemeRenderer(func(site *entities.Site, w io.Writer) error {
		tpl, err := pongo2.FromFile(filepath.Join(a.config.ThemePath.String(), "templates", "page.html"))
		if err != nil {
			return fmt.Errorf("could not load page template: %w", err)
		}

		out, err := tpl.Execute(a.getPongoContext(site))

		if err != nil {
			return fmt.Errorf("could not execute page template: %w", err)
		}
		w.Write([]byte(out))
		return nil
	})
}
