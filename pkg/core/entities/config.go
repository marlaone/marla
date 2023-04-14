package entities

import "github.com/marlaone/marla/pkg/core/fields"

type Config struct {
	ContentPath     fields.Path           `yaml:"content_path"`
	DataPath        fields.Path           `yaml:"data_path"`
	ThemePath       fields.Path           `yaml:"theme_path"`
	HttpHost        fields.RequiredString `yaml:"http_host"`
	HttpPort        fields.HttpPort       `yaml:"http_port"`
	DefaultLangauge fields.RequiredString `yaml:"default_language"`
}

func NewConfig(contentPath fields.Path, dataPath fields.Path, themePath fields.Path, httpHost fields.RequiredString, httpPort fields.HttpPort, defaultLanguage fields.RequiredString) *Config {
	return &Config{
		ContentPath:     contentPath,
		DataPath:        dataPath,
		ThemePath:       themePath,
		HttpHost:        httpHost,
		HttpPort:        httpPort,
		DefaultLangauge: defaultLanguage,
	}
}
