use crate::service::example_service::ExempleService;

/// Dependency to hold all Services. In order to be accessible in Controllers,
/// Services must implement the FromRef trait (see ExampleService)
#[derive(Clone)]
pub struct ServiceBag {
    pub example_service: ExampleService,
}

impl ServiceBag {
    pub fn new() -> Self {
        Self {
            example_service: ExampleService::new(),
        }
    }
}
