package ports

import (
	"io"

	"github.com/marlaone/marla/pkg/core/entities"
)

// ThemeRenderer is a function that renders a theme template.
type ThemeRenderer func(site *entities.Site, w io.Writer) error

// ThemePort is the port for the theme.
type ThemePort interface {
	TemplateRenderer() ThemeRenderer
	NotFoundRenderer() ThemeRenderer
	WatchTemplates()
	AddTemplateVariable(name string, value any)
}
