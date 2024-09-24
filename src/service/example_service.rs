use crate::config::error::{ApiResult, AppError};
use crate::domain::company_info::CompanyInfo;
use crate::service::web_scraping_service::WebScrapingService;
use crate::AppState;
use anyhow::anyhow;
use axum::extract::FromRef;
use regex::Regex;
use scraper::Html;
use url::Url;

#[derive(Clone, FromRef)]
pub struct ExampleService {
    // Add dependencies if necessary
}

impl ExampleService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn example_service_method(&self, url: &str) -> ApiResult<()> {
        Ok(())
    }
}

// All services must implement `FromRef` trait to allow DI
impl FromRef<AppState> for ExampleService {
    fn from_ref(state: &AppState) -> Self {
        state.service_bag.company_info_service.clone()
    }
}
