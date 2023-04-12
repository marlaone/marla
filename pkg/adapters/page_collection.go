package adapters

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/marlaone/marla/pkg/adapters/utils"
	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/fields"
	"github.com/marlaone/marla/pkg/core/ports"
)

type PageCollectionAdapter struct {
	collection *entities.PageCollection
}

var _ ports.PageCollectionPort = &PageCollectionAdapter{}

func NewPageCollectionAdapter() *PageCollectionAdapter {
	return &PageCollectionAdapter{}
}

func (a *PageCollectionAdapter) InitializePageCollection(config *entities.Config) error {

	a.collection = entities.NewPageCollection()

	err := filepath.Walk(config.ContentPath.String(), func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if info.IsDir() {
			return nil
		}

		if filepath.Ext(path) == ".md" {
			page, err := utils.PageFromMarkdownFile(config, fields.Path(path))
			if err != nil {
				return fmt.Errorf("could not create page from markdown file: %w", err)
			}
			a.collection.AddPage(page)
		}

		return nil
	})

	if err != nil {
		return fmt.Errorf("could not walk content path: %w", err)
	}

	return nil
}

func (a *PageCollectionAdapter) WatchPageCollection() error {
	return nil
}

func (a *PageCollectionAdapter) GetPageCollection() *entities.PageCollection {
	return a.collection
}

func (a *PageCollectionAdapter) RegisterPageCollectionObserver(observer entities.PageCollectionObserver) {
	a.collection.AddObserver(observer)
}

func (a *PageCollectionAdapter) UnregisterPageCollectionObserver(observer *entities.PageCollectionObserver) {
	a.collection.RemoveObserver(observer)
}
