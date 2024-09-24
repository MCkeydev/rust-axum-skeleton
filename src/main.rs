use crate::controller::company_info_controller;
use crate::service::service_bag::ServiceBag;
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod config;
mod controller;
mod domain;
mod dto;
mod service;

#[derive(Clone)]
struct AppState {
    service_bag: ServiceBag,
}

#[tokio::main]
async fn main() {
    // Configure and initialize the tracing subscriber
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Define shared state of the application
    // Add all your controllers/routers
    let app = Router::new()
        .nest("/", company_info_controller::router())
        .with_state(AppState {
            service_bag: ServiceBag::new(),
        })
        .fallback(|| async { "Page not found !" })
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
