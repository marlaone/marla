package main

import (
	"bytes"
	"fmt"
	"log"
	"net/url"

	"github.com/marlaone/marla/pkg/adapters"
	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/fields"
)

type testObserver struct{}

func (to *testObserver) AddedPage(page *entities.Page) {
	log.Println("Added page", page.Title, page.Extra)
}

func (to *testObserver) RemovedPage(page *entities.Page) {
	log.Println("Removed page", page.Title)
}

func (to *testObserver) UpdatedPage(page *entities.Page) {
	log.Println("Updated page", page.Title)
}

func main() {

	config := entities.NewConfig(
		fields.MustPathFromString("./site/content"),
		fields.MustPathFromString("./site/content"),
		fields.MustPathFromString("./site/themes/marla"),
		fields.MustRequiredStringFromString("localhost"),
		fields.MustHttpPortFromInt(1818),
		fields.MustRequiredStringFromString("en"),
	)

	pageCollectionAdapter := adapters.NewPageCollectionAdapter()

	pageCollectionAdapter.RegisterPageCollectionObserver(&testObserver{})

	if err := pageCollectionAdapter.InitializePageCollection(config); err != nil {
		log.Fatalln(err)
	}

	log.Println(len(pageCollectionAdapter.GetPageCollection().Pages()))

	go pageCollectionAdapter.WatchPageCollection()

	siteAdapter := adapters.NewSiteAdapter(config)

	url, err := url.Parse("http://localhost:1818/blog/my-love-to-rust-and-go")
	if err != nil {
		log.Fatalln(err)
	}

	site, err := siteAdapter.FetchSite(pageCollectionAdapter.GetPageCollection(), url, fields.MustRequiredStringFromString("de"))
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println(site.Page.Title)

	marlaTheme := adapters.NewMarlaThemeAdapter(config)

	var buf bytes.Buffer
	if err := marlaTheme.IndexRenderer()(site, &buf); err != nil {
		log.Fatalln(err)
	}

	fmt.Println(buf.String())

	<-make(chan struct{})
}
