package services

import (
	"fmt"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
)

// ConfigService is the service for the config.
// It implements the ConfigPort interface.
type ConfigService struct {
	adapter ports.ConfigPort
}

// NewConfigService returns a new ConfigService.
func NewConfigService(adapter ports.ConfigPort) *ConfigService {
	return &ConfigService{
		adapter: adapter,
	}
}

// LoadConfig loads the config from a file.
// It returns a LoadConfigError if an error occurs.
func (s *ConfigService) LoadConfig(filename string) (*entities.Config, error) {
	config, err := s.adapter.LoadConfig(filename)
	if err != nil {
		return nil, &LoadConfigError{Err: err}
	}

	return config, nil
}

// LoadConfigError is the error returned by LoadConfig.
type LoadConfigError struct {
	Err error
}

// Error returns the error message.
func (e *LoadConfigError) Error() string {
	return fmt.Sprintf("error loading config: %v", e.Err)
}
