package adapters

import (
	"fmt"
	"os"
	"path/filepath"
	"time"

	"github.com/marlaone/marla/pkg/adapters/utils"
	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/fields"
	"github.com/marlaone/marla/pkg/core/ports"
)

// PageCollectionAdapter implements the PageCollectionPort interface.
type PageCollectionAdapter struct {
	collection *entities.PageCollection
	config     *entities.Config
}

var _ ports.PageCollectionPort = &PageCollectionAdapter{}

// NewPageCollectionAdapter returns a new PageCollectionAdapter.
func NewPageCollectionAdapter() *PageCollectionAdapter {
	return &PageCollectionAdapter{
		collection: entities.NewPageCollection(),
	}
}

// InitializePageCollection initializes the page collection.
// It walks the content path and creates a page for each markdown file.
// It also sets the default language.
// It returns an error if the content path could not be walked.
// It returns an error if a page could not be created from a markdown file.
func (a *PageCollectionAdapter) InitializePageCollection(config *entities.Config) error {
	a.config = config
	a.collection.SetDefaultLanguage(config.DefaultLangauge)
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

// WatchPageCollection watches the content path for changes.
func (a *PageCollectionAdapter) WatchPageCollection() error {

	// map of file paths and their last modified time
	filesModified := map[string]time.Time{}

	// get the last modified time for each page file
	for _, p := range a.collection.Pages() {
		info, err := os.Stat(p.ContentPath.String())
		if err != nil {
			return fmt.Errorf("could not get file info for %s: %w", p.ContentPath.String(), err)
		}
		filesModified[p.ContentPath.String()] = info.ModTime()
	}

	errC := make(chan error)

	ticker := time.NewTicker(5 * time.Second)
	defer ticker.Stop()
	go func() {
		for {
			select {
			case <-ticker.C:
				// check if any file has been modified
				for path, modified := range filesModified {
					info, err := os.Stat(path)
					// if the file does not exist anymore, remove it from the collection
					if err != nil {
						a.collection.RemovePageByPath(fields.Path(path))
						delete(filesModified, path)
						continue
					}
					// if the file has been modified, update the page in the collection
					if info.ModTime() != modified {
						page, err := utils.PageFromMarkdownFile(a.config, fields.Path(path))
						if err != nil {
							errC <- fmt.Errorf("could not create page from markdown file: %w", err)
						}
						a.collection.UpdatePage(fields.Path(path), page)
						filesModified[path] = info.ModTime()
					}
				}

				// check if any new files have been added
				filepath.Walk(a.config.ContentPath.String(), func(path string, info os.FileInfo, err error) error {
					if err != nil {
						errC <- fmt.Errorf("could not walk content path: %w", err)
					}

					if info.IsDir() {
						return nil
					}

					if filepath.Ext(path) == ".md" {
						if _, ok := filesModified[path]; !ok {
							page, err := utils.PageFromMarkdownFile(a.config, fields.Path(path))
							if err == nil {
								a.collection.AddPage(page)
							}
							filesModified[path] = info.ModTime()
						}
					}

					return nil
				})
			}
		}
	}()

	return <-errC
}

// GetPageCollection returns the page collection.
func (a *PageCollectionAdapter) GetPageCollection() *entities.PageCollection {
	return a.collection
}

// RegisterPageCollectionObserver registers a page collection observer.
func (a *PageCollectionAdapter) RegisterPageCollectionObserver(observer entities.PageCollectionObserver) {
	a.collection.AddObserver(observer)
}

// UnregisterPageCollectionObserver unregisters a page collection observer.
func (a *PageCollectionAdapter) UnregisterPageCollectionObserver(observer *entities.PageCollectionObserver) {
	a.collection.RemoveObserver(observer)
}
