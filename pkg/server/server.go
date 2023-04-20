package server

import (
	"net/url"
	"path/filepath"
	"strconv"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/utils"
	"github.com/marlaone/marla/pkg/core"
	"github.com/marlaone/marla/pkg/core/entities"
)

// RouterBuilder is a function that builds the router.
// The function is called with the application, the config and the fiber app.
// The function should return an error if something went wrong.
type RouterBuilder func(app *core.Application, config *entities.Config, fiberapp *fiber.App) error

// Server represents the server.
type Server struct {
	fiberapp *fiber.App
	app      *core.Application
	config   *entities.Config
}

// NewServer creates a new server. The server is not started.
// The server is created with the given config and application.
func NewServer(config *entities.Config, app *core.Application) *Server {
	return &Server{
		fiberapp: fiber.New(fiber.Config{Prefork: false}),
		config:   config,
		app:      app,
	}
}

// CreateRouter creates the router with the given builder.
// The builder is called with the application, the config and the fiber app.
// The builder should return an error if something went wrong.
func (s *Server) CreateRouter(builder RouterBuilder) error {
	return builder(s.app, s.config, s.fiberapp)
}

// CreateDefaultRouter creates the default router.
// The default router serves the static files from the theme.
// Registers routes for all pages.
func (s *Server) CreateDefaultRouter() error {
	return s.CreateRouter(func(app *core.Application, config *entities.Config, fiberapp *fiber.App) error {
		fiberapp.Static("/", filepath.Join(config.ThemePath.String()))
		fiberapp.Static("/", filepath.Join(config.ThemePath.String(), "static"))

		fiberapp.Get("/*", func(c *fiber.Ctx) error {
			originalUrl := utils.CopyString(c.OriginalURL())
			uri, err := url.Parse(originalUrl)
			if err != nil {
				return err
			}

			// TODO: get language from request
			site, err := app.SiteService().FetchSite(uri, "en")
			if err != nil {
				return err
			}

			c.Context().SetContentType("text/html; charset=utf-8")
			if site.Page == nil {
				c.Status(fiber.StatusNotFound)
				if err := app.ThemeService().RenderNotFound(site, c); err != nil {
					return err
				}
				return nil
			}

			if err := app.ThemeService().RenderPage(site, c); err != nil {
				return err
			}

			return nil
		})

		return nil
	})
}

// Start starts the server.
func (s *Server) Start() error {
	return s.fiberapp.Listen(":" + strconv.Itoa(int(s.config.HttpPort)))
}
