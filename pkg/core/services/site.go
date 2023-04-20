package services

import (
	"fmt"
	"net/url"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/fields"
	"github.com/marlaone/marla/pkg/core/ports"
)

// SiteService is the service for the site.
type SiteService struct {
	siteAdapter           ports.SitePort
	pageCollectionAdapter ports.PageCollectionPort
}

// NewSiteService creates a new site service.
func NewSiteService(siteAdapter ports.SitePort, pageCollectionAdapter ports.PageCollectionPort) *SiteService {
	return &SiteService{
		siteAdapter:           siteAdapter,
		pageCollectionAdapter: pageCollectionAdapter,
	}
}

// FetchSite fetches a site based on the path and the user language.
func (s *SiteService) FetchSite(path *url.URL, userLanguage string) (*entities.Site, error) {

	language, err := fields.RequiredStringFromString(userLanguage)
	if err != nil {
		return nil, &InvalidLanguageError{Language: userLanguage}
	}

	site, err := s.siteAdapter.FetchSite(s.pageCollectionAdapter.GetPageCollection(), path, language)
	if err != nil {
		return nil, &FetchSiteError{Err: err}
	}

	return site, nil
}

// InvalidLanguageError is returned when the language is invalid.
type InvalidLanguageError struct {
	Language string
}

// Error returns the error message.
func (e InvalidLanguageError) Error() string {
	return fmt.Sprintf("invalid language: %s", e.Language)
}

// FetchSiteError is returned when the site could not be fetched.
type FetchSiteError struct {
	Err error
}

// Error returns the error message.
func (e FetchSiteError) Error() string {
	return fmt.Sprintf("could not fetch site: %v", e.Err)
}
