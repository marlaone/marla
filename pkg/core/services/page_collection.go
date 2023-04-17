package services

import (
	"fmt"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
)

type PageCollectionService struct {
	adapter ports.PageCollectionPort
}

func NewPageCollectionService(adapter ports.PageCollectionPort) *PageCollectionService {
	return &PageCollectionService{
		adapter: adapter,
	}
}

func (s *PageCollectionService) GetPageCollection() *entities.PageCollection {
	return s.adapter.GetPageCollection()
}

func (s *PageCollectionService) Watch() error {
	err := s.adapter.WatchPageCollection()
	if err != nil {
		return fmt.Errorf("[PageCollectionService.WatchPageChanges] %w", err)
	}
	return nil
}

func (s *PageCollectionService) Initialize(config *entities.Config) error {
	err := s.adapter.InitializePageCollection(config)
	if err != nil {
		return fmt.Errorf("[PageCollectionService.InitializePageCollection] %w", err)
	}
	return nil
}

func (s *PageCollectionService) Subscribe(observer entities.PageCollectionObserver) {
	s.adapter.RegisterPageCollectionObserver(observer)
}

func (s *PageCollectionService) Unsubscribe(observer *entities.PageCollectionObserver) {
	s.adapter.UnregisterPageCollectionObserver(observer)
}
