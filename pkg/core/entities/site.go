package entities

import (
	"net/url"

	"github.com/marlaone/marla/pkg/core/fields"
)

type Site struct {
	BaseURL  string
	Path     *url.URL
	Config   *Config
	Pages    []*Page
	Page     *Page
	Data     map[string]any
	Language fields.RequiredString
}
