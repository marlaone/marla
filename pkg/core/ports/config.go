package ports

import "github.com/marlaone/marla/pkg/core/entities"

type ConfigPort interface {
	LoadConfig(filename string) (*entities.Config, error)
}
