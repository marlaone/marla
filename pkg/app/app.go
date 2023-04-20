package app

import (
	"fmt"
	"log"
	"os"

	"github.com/marlaone/marla/pkg/adapters"
	"github.com/marlaone/marla/pkg/core"
	"github.com/marlaone/marla/pkg/core/services"
	"github.com/marlaone/marla/pkg/server"
)

// getExistingConfigPath returns the first existing config path.
func getExistingConfigPath(configDirs []string) string {
	for _, dir := range configDirs {
		if _, err := os.Stat(dir); err == nil {
			return dir
		}
	}
	return ""
}

// StartDefaultApplication starts the default application.
// It returns an error if the config could not be loaded.
// It returns an error if the page collection could not be initialized.
// It returns an error if the server could not be started.
func StartDefaultApplication() error {
	configAdapter := adapters.NewConfigAdapter()
	configService := services.NewConfigService(configAdapter)

	homedir, err := os.UserHomeDir()
	if err != nil {
		log.Printf("[App.DefaultApp] get user home dir: %v\n", err)
	}

	configDirs := []string{
		"./config.yml",
		"./site/config.yml",
	}
	if homedir != "" {
		configDirs = append(configDirs, homedir+"/.marla/config.yml")
	}

	configPath := getExistingConfigPath(configDirs)

	config, err := configService.LoadConfig(configPath)
	if err != nil {
		return fmt.Errorf("[App.DefaultApp] load config: %w", err)
	}

	pca := adapters.NewPageCollectionAdapter()
	app := core.NewApplication(
		services.NewPageCollectionService(pca),
		services.NewThemeService(adapters.NewThemeAdapter(config)),
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

	srv := server.NewServer(config, app)

	if err := srv.CreateDefaultRouter(); err != nil {
		return fmt.Errorf("[App.DefaultApp] create default router: %w", err)
	}

	if err := srv.Start(); err != nil {
		return fmt.Errorf("[App.DefaultApp] start server: %w", err)
	}

	return nil
}
