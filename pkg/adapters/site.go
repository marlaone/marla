package adapters

import (
	"net/url"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/fields"
	"github.com/marlaone/marla/pkg/core/ports"
)

// SiteAdapter implements the SitePort interface.
type SiteAdapter struct {
	config *entities.Config
}

var _ ports.SitePort = &SiteAdapter{}

// NewSiteAdapter returns a new SiteAdapter.
func NewSiteAdapter(config *entities.Config) *SiteAdapter {
	return &SiteAdapter{
		config: config,
	}
}

// FetchSite returns a new Site.
// The Site contains the current page and all pages.
// The Site also contains the config.
// The Site also contains the current path.
// The Site also contains the base URL.
// The Site also contains a map for custom data.
// The Site also contains the current language.
func (sa *SiteAdapter) FetchSite(collection *entities.PageCollection, path *url.URL, userLanguage fields.RequiredString) (*entities.Site, error) {
	site := &entities.Site{
		Path:    path,
		Config:  sa.config,
		Data:    make(map[string]any),
		Pages:   collection.Pages(),
		Page:    collection.GetPageByURLAndLanguage(path, userLanguage),
		BaseURL: sa.config.BaseURL,
	}
	return site, nil
}
