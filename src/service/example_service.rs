use crate::config::error::ApiResult;
use crate::domain::example::Example;
use crate::AppState;
use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct ExampleService {
    // Add dependencies if necessary
}

impl ExampleService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn example_service_method(&self) -> ApiResult<Example> {
        Ok(Example::new())
    }
}

// All services must implement `FromRef` trait to allow DI
impl FromRef<AppState> for ExampleService {
    fn from_ref(state: &AppState) -> Self {
        state.service_bag.example_service.clone()
    }
}
