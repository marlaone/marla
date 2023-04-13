package entities

import (
	"fmt"
	"net/url"

	"github.com/marlaone/marla/pkg/core/fields"
)

type PageCollectionObserver interface {
	AddedPage(page *Page)
	RemovedPage(page *Page)
	UpdatedPage(page *Page)
}

type PageCollection struct {
	pages           []*Page
	observers       []PageCollectionObserver
	defaultLanguage fields.RequiredString
}

func NewPageCollection() *PageCollection {
	return &PageCollection{
		pages:           make([]*Page, 0),
		observers:       make([]PageCollectionObserver, 0),
		defaultLanguage: "",
	}
}

func (pc *PageCollection) SetDefaultLanguage(defaultLanguage fields.RequiredString) {
	pc.defaultLanguage = defaultLanguage
}

func (pc *PageCollection) GetPageByURL(url url.URL) *Page {
	for _, page := range pc.pages {
		if page.Path.Path == url.Path {
			return page
		}
	}
	return nil
}

func (pc *PageCollection) GetPageByURLAndLanguage(url *url.URL, language fields.RequiredString) *Page {
	uri := url.Path
	if uri == "" {
		uri = "/"
	}
	fmt.Println(uri, language)
	for _, page := range pc.pages {
		fmt.Println(page.Path, page.Language)
		if page.Path.Path == uri && page.Language == language {
			return page
		}
	}
	for _, page := range pc.pages {
		if page.Path.Path == uri && page.Language == pc.defaultLanguage {
			return page
		}
	}
	return nil
}

func (pc *PageCollection) AddPage(page *Page) {
	pc.pages = append(pc.pages, page)
	for _, observer := range pc.observers {
		observer.AddedPage(page)
	}
}

func (pc *PageCollection) RemovePage(page *Page) {
	for i, p := range pc.pages {
		if p == page {
			pc.pages = append(pc.pages[:i], pc.pages[i+1:]...)
			for _, observer := range pc.observers {
				observer.RemovedPage(page)
			}
			break
		}
	}
}

func (pc *PageCollection) RemovePageByPath(contentPath fields.Path) {
	for _, page := range pc.pages {
		if page.ContentPath == contentPath {
			pc.RemovePage(page)
			break
		}
	}
}

func (pc *PageCollection) UpdatePage(path fields.Path, p *Page) {
	updated := false
	for i, page := range pc.pages {
		if page.ContentPath == path {
			pc.pages[i] = p
			updated = true
			for _, observer := range pc.observers {
				observer.UpdatedPage(p)
			}
			break
		}
	}

	if !updated {
		pc.AddPage(p)
	}
}

func (pc *PageCollection) Pages() []*Page {
	return pc.pages
}

func (pc *PageCollection) AddObserver(observer PageCollectionObserver) {
	pc.observers = append(pc.observers, observer)
}

func (pc *PageCollection) RemoveObserver(observer *PageCollectionObserver) {
	for i, o := range pc.observers {
		if &o == observer {
			pc.observers = append(pc.observers[:i], pc.observers[i+1:]...)
			break
		}
	}
}
