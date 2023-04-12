use crate::entities::{config::Config, page::Page, page_collection::PageCollection};

pub enum PageCollectionAdapterError {
    NotFound,
    Unknown,
}

pub enum PageChangeType {
    Added,
    Removed,
    Updated,
}

pub type PageChangeSubscriber = fn(PageChangeType, &Page);

pub fn are_subscribers_eq<T: ?Sized>(left: &Box<T>, right: &Box<T>) -> bool {
    let left: *const T = left.as_ref();
    let right: *const T = right.as_ref();
    left == right
}

pub trait PageCollectionPort {
    fn create_page_collection(
        &mut self,
        config: Config,
    ) -> Result<PageCollection, PageCollectionAdapterError>;

    fn fetch_page_collection(&self) -> Result<&PageCollection, PageCollectionAdapterError>;

    fn watch_page_changes(&self) -> Result<(), PageCollectionAdapterError>;

    fn subscribe_to_page_changes(
        &mut self,
        callback: PageChangeSubscriber,
    ) -> Result<(), PageCollectionAdapterError>;

    fn unsubscribe_from_page_changes(
        &mut self,
        callback: PageChangeSubscriber,
    ) -> Result<(), PageCollectionAdapterError>;
}
