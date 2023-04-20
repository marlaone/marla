package entities

import (
	"net/url"

	"github.com/marlaone/marla/pkg/core/fields"
)

// Site represents the site.
// The Site contains the current page and all pages.
// The Site also contains the config.
// The Site also contains the current path.
// The Site also contains the base URL.
// The Site also contains a map for custom data.
// The Site also contains the current language.
type Site struct {
	BaseURL  string
	Path     *url.URL
	Config   *Config
	Pages    []*Page
	Page     *Page
	Data     map[string]any
	Language fields.RequiredString
}
