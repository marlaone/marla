use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

use crate::config::SETTINGS;

#[derive(Clone, Default)]
pub struct Database {}
impl Database {
    pub fn new() -> Database {
        Database {}
    }
}

// To make our Database usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Database {}

// Queries represent the callable funcitons
pub struct Query;
#[graphql_object(context = Database)]
impl Query {
    fn api_version() -> &'static str {
        let version = SETTINGS
            .read()
            .unwrap()
            .get_string("version")
            .unwrap_or("ERR".to_string())
            .to_owned();

        return Box::leak(version.into_boxed_str());
    }

    fn user(
        context: &Database,
        #[graphql(description = "id of the user")] id: i32,
    ) -> Option<String> {
        Some("yep".to_owned())
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}
