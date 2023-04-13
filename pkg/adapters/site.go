package adapters

import (
	"net/url"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/fields"
	"github.com/marlaone/marla/pkg/core/ports"
)

type SiteAdapter struct {
	config *entities.Config
}

var _ ports.SitePort = &SiteAdapter{}

func NewSiteAdapter(config *entities.Config) *SiteAdapter {
	return &SiteAdapter{
		config: config,
	}
}

func (sa *SiteAdapter) FetchSite(collection *entities.PageCollection, path *url.URL, userLanguage fields.RequiredString) (*entities.Site, error) {
	site := &entities.Site{
		Path:   path,
		Config: sa.config,
		Data:   make(map[string]any),
		Pages:  collection.Pages(),
		Page:   collection.GetPageByURLAndLanguage(path, userLanguage),
	}
	return site, nil
}
