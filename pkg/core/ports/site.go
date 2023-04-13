package ports

import (
	"net/url"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/fields"
)

type SitePort interface {
	FetchSite(collection *entities.PageCollection, path *url.URL, userLanguage fields.RequiredString) (*entities.Site, error)
}
