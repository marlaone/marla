package ports

import (
	"io"

	"github.com/marlaone/marla/pkg/core/entities"
)

type ThemeRenderer func(site *entities.Site, w io.Writer) error

type ThemePort interface {
	TemplateRenderer() ThemeRenderer
	NotFoundRenderer() ThemeRenderer
	WatchTemplates()
	AddTemplateVariable(name string, value any)
}
