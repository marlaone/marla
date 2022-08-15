use juniper::{EmptyMutation, EmptySubscription, RootNode};

use super::{context::GQLContext, query::Query};

pub type Schema =
    RootNode<'static, Query, EmptyMutation<GQLContext>, EmptySubscription<GQLContext>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<GQLContext>::new(),
        EmptySubscription::<GQLContext>::new(),
    )
}
