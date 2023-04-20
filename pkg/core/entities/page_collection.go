package entities

import (
	"net/url"
	"path"
	"strings"

	"github.com/marlaone/marla/pkg/core/fields"
)

// PageCollectionObserver is an interface for observing changes to a PageCollection.
type PageCollectionObserver interface {
	AddedPage(page *Page)
	RemovedPage(page *Page)
	UpdatedPage(page *Page)
}

// PageCollection represents a collection of pages.
// The PageCollection is used to store all pages.
// The PageCollection is used to get a page by its URL.
// The PageCollection is used to get a page by its URL and language.
// The PageCollection is used to get all pages.
type PageCollection struct {
	pages           []*Page
	observers       []PageCollectionObserver
	defaultLanguage fields.RequiredString
}

// NewPageCollection returns a new PageCollection.
func NewPageCollection() *PageCollection {
	return &PageCollection{
		pages:           make([]*Page, 0),
		observers:       make([]PageCollectionObserver, 0),
		defaultLanguage: "",
	}
}

// SetDefaultLanguage sets the default language.
func (pc *PageCollection) SetDefaultLanguage(defaultLanguage fields.RequiredString) {
	pc.defaultLanguage = defaultLanguage
}

// GetPageByURL returns a page by its URL.
func (pc *PageCollection) GetPageByURL(url url.URL) *Page {
	for _, page := range pc.pages {
		if page.Path.Path == url.Path {
			return page
		}
	}
	return nil
}

// GetPageByURLAndLanguage returns a page by its URL and language.
func (pc *PageCollection) GetPageByURLAndLanguage(url *url.URL, language fields.RequiredString) *Page {
	uri := url.Path
	if uri == "" {
		uri = "/"
	}

	// First try to find a page with the same language.
	for _, page := range pc.pages {
		if page.Path.Path == uri && page.Language == language {
			return page
		}
		// Check if the page has an alias with the same language.
		for _, alias := range page.Aliases {
			if alias == uri && page.Language == language {
				return page
			}
		}
	}

	// If no page with the same language was found, try to find a page with the default language.
	for _, page := range pc.pages {
		if page.Path.Path == uri && page.Language == pc.defaultLanguage {
			return page
		}
		// Check if the page has an alias with the default language.
		for _, alias := range page.Aliases {
			if alias == uri && page.Language == pc.defaultLanguage {
				return page
			}
		}
	}
	return nil
}

// AddPage adds a page to the PageCollection.
func (pc *PageCollection) AddPage(page *Page) {
	pc.pages = append(pc.pages, page)

	for _, p := range pc.pages {
		// Check if the page is a child of the current page.
		if page.Path.Path != "/" && path.Dir(page.Path.Path) == p.Path.Path {
			p.Children = append(p.Children, page)
			page.Parent = p
			break
		}
		// Check if the page is a parent of the current page.
		if strings.HasPrefix(p.Path.Path, page.Path.Path) {
			if page.Path.Path == "/" && p.Path.Path != "/" && strings.Count(p.Path.Path, "/") == 1 {
				page.Children = append(page.Children, p)
				p.Parent = page
				continue
			}

			trimmedPath := strings.TrimPrefix(p.Path.Path, page.Path.Path)
			if trimmedPath != "" && strings.Count(trimmedPath, "/") == 1 && strings.HasPrefix(trimmedPath, "/") {
				page.Children = append(page.Children, p)
				p.Parent = page
				continue
			}
		}
	}

	// Notify observers.
	for _, observer := range pc.observers {
		observer.AddedPage(page)
	}
}

// RemovePage removes a page from the PageCollection.
func (pc *PageCollection) RemovePage(page *Page) {
	for i, p := range pc.pages {
		if p == page {
			// Remove the page from its parent.
			if p.Parent != nil && len(p.Parent.Children) > 0 {
				for j, child := range p.Parent.Children {
					if child == page {
						p.Parent.Children = append(p.Parent.Children[:j], p.Parent.Children[j+1:]...)
						break
					}
				}
			}
			// Remove page from page collection.
			pc.pages = append(pc.pages[:i], pc.pages[i+1:]...)
			// Notify observers.
			for _, observer := range pc.observers {
				observer.RemovedPage(page)
			}
			break
		}
	}
}

// RemovePageByPath removes a page from the PageCollection by its path.
func (pc *PageCollection) RemovePageByPath(contentPath fields.Path) {
	for _, page := range pc.pages {
		if page.ContentPath == contentPath {
			pc.RemovePage(page)
			break
		}
	}
}

// UpdatePage updates a page in the PageCollection.
func (pc *PageCollection) UpdatePage(path fields.Path, p *Page) {
	updated := false
	for i, page := range pc.pages {
		if page.ContentPath == path {
			pc.pages[i] = p
			updated = true
			// Notify observers.
			for _, observer := range pc.observers {
				observer.UpdatedPage(p)
			}
			break
		}
	}

	// If the page was not found, add it.
	if !updated {
		pc.AddPage(p)
	}
}

// Pages returns all pages.
func (pc *PageCollection) Pages() []*Page {
	return pc.pages
}

// AddObserver adds an observer to the PageCollection.
func (pc *PageCollection) AddObserver(observer PageCollectionObserver) {
	pc.observers = append(pc.observers, observer)
}

// RemoveObserver removes an observer from the PageCollection.
func (pc *PageCollection) RemoveObserver(observer *PageCollectionObserver) {
	for i, o := range pc.observers {
		if &o == observer {
			pc.observers = append(pc.observers[:i], pc.observers[i+1:]...)
			break
		}
	}
}
