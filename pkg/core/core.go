package core

import "github.com/marlaone/marla/pkg/core/services"

type Application struct {
	pageCollectionService *services.PageCollectionService
	themeService          *services.ThemeService
	siteService           *services.SiteService
	configService         *services.ConfigService
}

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

func (a *Application) PageCollectionService() *services.PageCollectionService {
	return a.pageCollectionService
}

func (a *Application) ThemeService() *services.ThemeService {
	return a.themeService
}

func (a *Application) SiteService() *services.SiteService {
	return a.siteService
}

func (a *Application) ConfigService() *services.ConfigService {
	return a.configService
}
