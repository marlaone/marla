package services

import (
	"fmt"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
)

// PageCollectionService is the service for the page collection.
type PageCollectionService struct {
	adapter ports.PageCollectionPort
}

// NewPageCollectionService returns a new PageCollectionService.
func NewPageCollectionService(adapter ports.PageCollectionPort) *PageCollectionService {
	return &PageCollectionService{
		adapter: adapter,
	}
}

// GetPageCollection returns the page collection.
func (s *PageCollectionService) GetPageCollection() *entities.PageCollection {
	return s.adapter.GetPageCollection()
}

// Watch watches the page collection for changes.
func (s *PageCollectionService) Watch() error {
	err := s.adapter.WatchPageCollection()
	if err != nil {
		return fmt.Errorf("[PageCollectionService.WatchPageChanges] %w", err)
	}
	return nil
}

// Initialize initializes the page collection.
func (s *PageCollectionService) Initialize(config *entities.Config) error {
	err := s.adapter.InitializePageCollection(config)
	if err != nil {
		return fmt.Errorf("[PageCollectionService.InitializePageCollection] %w", err)
	}
	return nil
}

// Subscribe subscribes an observer to the page collection.
func (s *PageCollectionService) Subscribe(observer entities.PageCollectionObserver) {
	s.adapter.RegisterPageCollectionObserver(observer)
}

// Unsubscribe unsubscribes an observer from the page collection.
func (s *PageCollectionService) Unsubscribe(observer *entities.PageCollectionObserver) {
	s.adapter.UnregisterPageCollectionObserver(observer)
}
