package services

import (
	"fmt"
	"net/url"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/fields"
	"github.com/marlaone/marla/pkg/core/ports"
)

type SiteService struct {
	siteAdapter           ports.SitePort
	pageCollectionAdapter ports.PageCollectionPort
}

func NewSiteService(siteAdapter ports.SitePort, pageCollectionAdapter ports.PageCollectionPort) *SiteService {
	return &SiteService{
		siteAdapter:           siteAdapter,
		pageCollectionAdapter: pageCollectionAdapter,
	}
}

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

type InvalidLanguageError struct {
	Language string
}

func (e InvalidLanguageError) Error() string {
	return fmt.Sprintf("invalid language: %s", e.Language)
}

type FetchSiteError struct {
	Err error
}

func (e FetchSiteError) Error() string {
	return fmt.Sprintf("could not fetch site: %v", e.Err)
}
