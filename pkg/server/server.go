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

type RouterBuilder func(app *core.Application, config *entities.Config, fiberapp *fiber.App) error

type Server struct {
	fiberapp *fiber.App
	app      *core.Application
	config   *entities.Config
}

func NewServer(config *entities.Config, app *core.Application) *Server {
	return &Server{
		fiberapp: fiber.New(fiber.Config{Prefork: false}),
		config:   config,
		app:      app,
	}
}

func (s *Server) CreateRouter(builder RouterBuilder) error {
	return builder(s.app, s.config, s.fiberapp)
}

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

func (s *Server) Start() error {
	return s.fiberapp.Listen(":" + strconv.Itoa(int(s.config.HttpPort)))
}
