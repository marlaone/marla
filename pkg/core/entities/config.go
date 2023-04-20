package entities

import "github.com/marlaone/marla/pkg/core/fields"

// Config represents the configuration of the site.
type Config struct {
	ContentPath     fields.Path           `yaml:"content_path"`
	DataPath        fields.Path           `yaml:"data_path"`
	ThemePath       fields.Path           `yaml:"theme_path"`
	HttpHost        fields.RequiredString `yaml:"http_host"`
	HttpPort        fields.HttpPort       `yaml:"http_port"`
	DefaultLangauge fields.RequiredString `yaml:"default_language"`
	BaseURL         string                `yaml:"base_url"`
}
