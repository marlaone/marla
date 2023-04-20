package entities

import (
	"net/url"
	"time"

	"github.com/marlaone/marla/pkg/core/fields"
)

type Page struct {
	Parent   *Page
	Children []*Page

	Title          fields.RequiredString
	Path           *url.URL
	ContentPath    fields.Path
	Content        []byte
	PlainContent   []byte
	CreatedAt      time.Time
	LastModifiedAt *time.Time
	Words          uint
	Description    string
	Tags           []string
	Slug           string
	Aliases        []string
	Template       fields.Path
	Authors        []string
	IsDraft        bool
	Extra          map[string]any
	Language       fields.RequiredString
}
