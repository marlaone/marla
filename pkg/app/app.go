package app

import (
	"fmt"

	"github.com/marlaone/marla/pkg/adapters"
	"github.com/marlaone/marla/pkg/core"
	"github.com/marlaone/marla/pkg/core/services"
	"github.com/marlaone/marla/pkg/server"
	"github.com/spf13/viper"
)

// StartDefaultApplication starts the default application.
// It returns an error if the config could not be loaded.
// It returns an error if the page collection could not be initialized.
// It returns an error if the server could not be started.
func StartDefaultApplication() error {
	configAdapter := adapters.NewConfigAdapter()
	configService := services.NewConfigService(configAdapter)

	logger := adapters.NewLoggerAdapter()

	configPath := viper.GetViper().ConfigFileUsed()

	config, err := configService.LoadConfig(configPath)
	if err != nil {
		return fmt.Errorf("[App.DefaultApp] load config: %w", err)
	}

	pca := adapters.NewPageCollectionAdapter(logger)
	app := core.NewApplication(
		services.NewPageCollectionService(pca, logger),
		services.NewThemeService(adapters.NewThemeAdapter(config, logger)),
		services.NewSiteService(
			adapters.NewSiteAdapter(config),
			pca,
		),
		configService,
	)

	if err := app.PageCollectionService().Initialize(config); err != nil {
		return fmt.Errorf("[App.DefaultApp] initialize page collection: %w", err)
	}

	go app.PageCollectionService().Watch()
	go app.ThemeService().WatchTemplates()

	srv := server.NewServer(config, app, logger)

	if err := srv.CreateDefaultRouter(); err != nil {
		return fmt.Errorf("[App.DefaultApp] create default router: %w", err)
	}

	if err := srv.Start(); err != nil {
		return fmt.Errorf("[App.DefaultApp] start server: %w", err)
	}

	return nil
}
