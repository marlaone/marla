package main

import (
	"github.com/marlaone/marla/pkg/app"
)

func main() {
	if err := app.StartDefaultApplication(); err != nil {
		panic(err)
	}
}
