use crate::entities::{config::Config, page_collection::PageCollection};

pub enum PageCollectionAdapterError {
    NotFound,
    Unknown,
}

pub trait PageCollectionPort {
    fn create_page_collection(
        &self,
        config: Config,
    ) -> Result<PageCollection, PageCollectionAdapterError>;

    fn fetch_page_collection(&self) -> Result<&PageCollection, PageCollectionAdapterError>;
}
