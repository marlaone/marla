package services

import (
	"fmt"
	"io"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
)

// ThemeService is the service for the theme.
type ThemeService struct {
	adapter          ports.ThemePort
	templateRenderer ports.ThemeRenderer
}

// NewThemeService returns a new ThemeService.
func NewThemeService(adapter ports.ThemePort) *ThemeService {
	return &ThemeService{
		adapter:          adapter,
		templateRenderer: adapter.TemplateRenderer(),
	}
}

// RenderPage renders a page with the site data.
func (s *ThemeService) RenderPage(site *entities.Site, w io.Writer) error {
	err := s.templateRenderer(site, w)
	if err != nil {
		return fmt.Errorf("[ThemeService.RenderPage] %w", err)
	}
	return nil
}

// RenderNotFound renders the 404 page.
func (s *ThemeService) RenderNotFound(site *entities.Site, w io.Writer) error {
	err := s.adapter.NotFoundRenderer()(site, w)
	if err != nil {
		return fmt.Errorf("[ThemeService.RenderNotFound] %w", err)
	}
	return nil
}

// WatchTemplates watches the theme templates for changes.
func (a *ThemeService) WatchTemplates() {
	a.adapter.WatchTemplates()
}
