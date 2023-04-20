package adapters

import (
	"net/url"

	"github.com/marlaone/marla/pkg/adapters/utils"
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
	requestUri, _ := utils.GetPageURL(sa.config.BaseURL, path.Path)
	site := &entities.Site{
		Path:    path,
		Config:  sa.config,
		Data:    make(map[string]any),
		Pages:   collection.Pages(),
		Page:    collection.GetPageByURLAndLanguage(requestUri, userLanguage),
		BaseURL: sa.config.BaseURL,
	}
	return site, nil
}
