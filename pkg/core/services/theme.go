package services

import (
	"fmt"
	"io"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
)

type ThemeService struct {
	adapter       ports.ThemePort
	indexRenderer ports.ThemeRenderer
	pageRenderer  ports.ThemeRenderer
}

func NewThemeService(adapter ports.ThemePort) *ThemeService {
	return &ThemeService{
		adapter:       adapter,
		indexRenderer: adapter.IndexRenderer(),
		pageRenderer:  adapter.PageRenderer(),
	}
}

func (s *ThemeService) RenderIndexPage(site *entities.Site, w io.Writer) error {
	err := s.indexRenderer(site, w)
	if err != nil {
		return fmt.Errorf("[ThemeService.RenderIndexPage] %w", err)
	}
	return nil
}

func (s *ThemeService) RenderPage(site *entities.Site, w io.Writer) error {
	err := s.pageRenderer(site, w)
	if err != nil {
		return fmt.Errorf("[ThemeService.RenderPage] %w", err)
	}
	return nil
}
