package main

import (
	"log"

	"github.com/marlaone/marla/pkg/adapters"
	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/fields"
)

func main() {

	config := entities.NewConfig(
		fields.MustPathFromString("./site/content"),
		fields.MustPathFromString("./site/content"),
		fields.MustPathFromString("./site/themes/marla"),
		fields.MustRequiredStringFromString("localhost"),
		fields.MustHttpPortFromInt(1818),
	)

	pageCollectionAdapter := adapters.NewPageCollectionAdapter()

	if err := pageCollectionAdapter.InitializePageCollection(config); err != nil {
		log.Fatalln(err)
	}

	log.Println(pageCollectionAdapter.GetPageCollection())
}
