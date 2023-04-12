package entities

type PageCollectionObserver interface {
	AddedPage(page *Page)
	RemovedPage(page *Page)
	UpdatedPage(page *Page)
}

type PageCollection struct {
	pages     []Page
	observers []PageCollectionObserver
}

func NewPageCollection() *PageCollection {
	return &PageCollection{
		pages:     make([]Page, 0),
		observers: make([]PageCollectionObserver, 0),
	}
}

func (pc *PageCollection) AddPage(page Page) {
	pc.pages = append(pc.pages, page)
	for _, observer := range pc.observers {
		observer.AddedPage(&page)
	}
}

func (pc *PageCollection) RemovePage(page *Page) {
	for i, p := range pc.pages {
		if &p == page {
			pc.pages = append(pc.pages[:i], pc.pages[i+1:]...)
			for _, observer := range pc.observers {
				observer.RemovedPage(page)
			}
			break
		}
	}
}

func (pc *PageCollection) Pages() []Page {
	return pc.pages
}

func (pc *PageCollection) AddObserver(observer PageCollectionObserver) {
	pc.observers = append(pc.observers, observer)
}

func (pc *PageCollection) RemoveObserver(observer *PageCollectionObserver) {
	for i, o := range pc.observers {
		if &o == observer {
			pc.observers = append(pc.observers[:i], pc.observers[i+1:]...)
			break
		}
	}
}
