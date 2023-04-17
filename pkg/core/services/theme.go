package services

import (
	"fmt"
	"io"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
)

type ThemeService struct {
	adapter          ports.ThemePort
	templateRenderer ports.ThemeRenderer
}

func NewThemeService(adapter ports.ThemePort) *ThemeService {
	return &ThemeService{
		adapter:          adapter,
		templateRenderer: adapter.TemplateRenderer(),
	}
}

func (s *ThemeService) RenderPage(site *entities.Site, w io.Writer) error {
	err := s.templateRenderer(site, w)
	if err != nil {
		return fmt.Errorf("[ThemeService.RenderPage] %w", err)
	}
	return nil
}

func (s *ThemeService) RenderNotFound(site *entities.Site, w io.Writer) error {
	err := s.adapter.NotFoundRenderer()(site, w)
	if err != nil {
		return fmt.Errorf("[ThemeService.RenderNotFound] %w", err)
	}
	return nil
}

func (a *ThemeService) WatchTemplates() {
	a.adapter.WatchTemplates()
}
