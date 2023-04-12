use marla_core::ports::page_collection::{
    PageChangeSubscriber, PageCollectionAdapterError, PageCollectionPort,
};

pub struct PageCollectionAdapter {
    collection: marla_core::entities::page_collection::PageCollection,
    subscribers: Vec<PageChangeSubscriber>,
}

impl PageCollectionAdapter {
    pub fn new() -> Self {
        Self {
            collection: marla_core::entities::page_collection::PageCollection::new(),
            subscribers: Vec::new(),
        }
    }

    fn load_pages(&mut self) -> Result<(), PageCollectionAdapterError> {
        unimplemented!()
    }
}

impl PageCollectionPort for PageCollectionAdapter {
    fn create_page_collection(
        &mut self,
        config: marla_core::entities::config::Config,
    ) -> Result<
        marla_core::entities::page_collection::PageCollection,
        marla_core::ports::page_collection::PageCollectionAdapterError,
    > {
        self.collection = marla_core::entities::page_collection::PageCollection::new();
        return Ok(self.collection.clone());
    }

    fn fetch_page_collection(
        &self,
    ) -> Result<
        &marla_core::entities::page_collection::PageCollection,
        marla_core::ports::page_collection::PageCollectionAdapterError,
    > {
        return Ok(&self.collection);
    }

    fn watch_page_changes(
        &self,
    ) -> Result<(), marla_core::ports::page_collection::PageCollectionAdapterError> {
        unimplemented!()
    }

    fn subscribe_to_page_changes(
        &mut self,
        callback: PageChangeSubscriber,
    ) -> Result<(), PageCollectionAdapterError> {
        self.subscribers.push(callback);
        Ok(())
    }

    fn unsubscribe_from_page_changes(
        &mut self,
        callback: PageChangeSubscriber,
    ) -> Result<(), PageCollectionAdapterError> {
        self.subscribers
            .retain(|subscriber| subscriber as *const PageChangeSubscriber != &callback);
        Ok(())
    }
}
