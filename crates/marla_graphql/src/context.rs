#[derive(Clone, Default)]
pub struct GQLContext {}
impl GQLContext {
    pub fn new() -> GQLContext {
      GQLContext {}
    }
}

// To make our GQLContext usable by Juniper, we have to implement a marker trait.
impl juniper::Context for GQLContext {}