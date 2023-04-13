package entities

import "github.com/marlaone/marla/pkg/core/fields"

type Config struct {
	ContentPath     fields.Path
	DataPath        fields.Path
	ThemePath       fields.Path
	HttpHost        fields.RequiredString
	HttpPort        fields.HttpPort
	DefaultLangauge fields.RequiredString
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
