package adapters

import (
	"fmt"
	"io"
	"path/filepath"

	"github.com/flosch/pongo2/v6"
	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
)

type ThemeAdapter struct {
	config *entities.Config
}

var _ ports.ThemePort = &ThemeAdapter{}

func NewMarlaThemeAdapter(config *entities.Config) *ThemeAdapter {
	return &ThemeAdapter{
		config: config,
	}
}

func (a *ThemeAdapter) IndexRenderer() ports.ThemeRenderer {
	pongo2.RegisterFilter("truncate", func(in *pongo2.Value, param *pongo2.Value) (*pongo2.Value, *pongo2.Error) {
		return pongo2.AsValue(in.String()[:param.Integer()]), nil
	})

	return ports.ThemeRenderer(func(site *entities.Site, w io.Writer) error {
		tpl, err := pongo2.FromFile(filepath.Join(a.config.ThemePath.String(), "templates", "index.html"))
		if err != nil {
			return fmt.Errorf("could not load index template: %w", err)
		}
		out, err := tpl.Execute(pongo2.Context{
			"site": site,
			"bytes_to_string": func(b []byte) string {
				return string(b)
			},
		})
		if err != nil {
			return fmt.Errorf("could not execute index template: %w", err)
		}
		w.Write([]byte(out))
		return nil
	})
}

func (a *ThemeAdapter) PageRenderer() ports.ThemeRenderer {
	return ports.ThemeRenderer(func(site *entities.Site, w io.Writer) error {
		tpl, err := pongo2.FromFile(filepath.Join(a.config.ThemePath.String(), "templates", "page.html"))
		if err != nil {
			return fmt.Errorf("could not load page template: %w", err)
		}
		out, err := tpl.Execute(pongo2.Context{
			"site": site,
			"bytes_to_string": func(b []byte) string {
				return string(b)
			},
		})
		if err != nil {
			return fmt.Errorf("could not execute page template: %w", err)
		}
		w.Write([]byte(out))
		return nil
	})
}
