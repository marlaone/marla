package services

import (
	"fmt"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/ports"
)

type ConfigService struct {
	adapter ports.ConfigPort
}

func NewConfigService(adapter ports.ConfigPort) *ConfigService {
	return &ConfigService{
		adapter: adapter,
	}
}

func (s *ConfigService) LoadConfig(filename string) (*entities.Config, error) {
	config, err := s.adapter.LoadConfig(filename)
	if err != nil {
		return nil, &LoadConfigError{Err: err}
	}

	return config, nil
}

type LoadConfigError struct {
	Err error
}

func (e *LoadConfigError) Error() string {
	return fmt.Sprintf("error loading config: %v", e.Err)
}
