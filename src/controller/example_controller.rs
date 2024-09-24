use crate::config::error::ApiResult;
use crate::domain::example::Example;
use crate::dto::example_dto::ExampleDto;
use crate::service::example_service::ExampleService;
use crate::AppState;
use axum::extract::{Query, State};
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use axum_valid::Valid;

/// Router<S> where S is the type of the global state (not the sub-states used in your controller)
pub fn router() -> Router<AppState> {
    Router::new().route("/example", get(example_controller_route))
    // TODO: Add your routes here
}

/// Attempt to find company information
#[debug_handler]
pub async fn example_controller_route(
    State(example_service): State<ExampleService>,
    Valid(Query(_params)): Valid<Query<ExampleDto>>,
) -> ApiResult<Json<Example>> {
    let example = example_service.example_service_method().await?;

    Ok(Json(example))
}
