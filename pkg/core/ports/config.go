package ports

import "github.com/marlaone/marla/pkg/core/entities"

// ConfigPort is the port for the config.
type ConfigPort interface {
	LoadConfig(filename string) (*entities.Config, error)
}
