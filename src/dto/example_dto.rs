use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct ExampleDto {
    #[validate(url)]
    pub url: String,
}
