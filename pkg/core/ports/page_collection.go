package ports

import "github.com/marlaone/marla/pkg/core/entities"

type PageCollectionPort interface {
	InitializePageCollection(config *entities.Config) error
	WatchPageCollection() error
	GetPageCollection() *entities.PageCollection
	RegisterPageCollectionObserver(observer entities.PageCollectionObserver)
	UnregisterPageCollectionObserver(observer *entities.PageCollectionObserver)
}
