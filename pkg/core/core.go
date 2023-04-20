package core

import "github.com/marlaone/marla/pkg/core/services"

// Application is the core application.
// It contains all services and ports.
type Application struct {
	pageCollectionService *services.PageCollectionService
	themeService          *services.ThemeService
	siteService           *services.SiteService
	configService         *services.ConfigService
}

// NewApplication returns a new Application.
func NewApplication(
	pageCollectionService *services.PageCollectionService,
	themeService *services.ThemeService,
	siteService *services.SiteService,
	configService *services.ConfigService,
) *Application {
	return &Application{
		pageCollectionService: pageCollectionService,
		themeService:          themeService,
		siteService:           siteService,
		configService:         configService,
	}
}

// PageCollectionService returns the page collection service.
func (a *Application) PageCollectionService() *services.PageCollectionService {
	return a.pageCollectionService
}

// ThemeService returns the theme service.
func (a *Application) ThemeService() *services.ThemeService {
	return a.themeService
}

// SiteService returns the site service.
func (a *Application) SiteService() *services.SiteService {
	return a.siteService
}

// ConfigService returns the config service.
func (a *Application) ConfigService() *services.ConfigService {
	return a.configService
}
