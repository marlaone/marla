package adapters

import (
	"fmt"
	"os"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
	"gopkg.in/yaml.v3"
)

// ConfigAdapter implements the ConfigPort interface.
type ConfigAdapter struct {
}

var _ ports.ConfigPort = &ConfigAdapter{}

// NewConfigAdapter returns a new ConfigAdapter.
func NewConfigAdapter() *ConfigAdapter {
	return &ConfigAdapter{}
}

// LoadConfig loads the config from a file.
func (a *ConfigAdapter) LoadConfig(filename string) (*entities.Config, error) {

	configFile, err := os.ReadFile(filename)
	if err != nil {
		return nil, fmt.Errorf("[ConfigAdapter.LoadConfig] read config file: %w", err)
	}

	config := entities.Config{}
	err = yaml.Unmarshal(configFile, &config)
	if err != nil {
		return nil, fmt.Errorf("[ConfigAdapter.LoadConfig] unmarshal config: %w", err)
	}

	return &config, nil
}
