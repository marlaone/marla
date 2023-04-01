use crate::entities::page_collection::PageCollection;

pub enum CreatePageCollectionError {
    Unknown,
}

pub fn create_page_collection() -> Result<PageCollection, CreatePageCollectionError> {
    Ok(PageCollection { pages: vec![] })
}
