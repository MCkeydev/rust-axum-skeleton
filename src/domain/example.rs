use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize)]
pub struct Example {
    pub phone_number: HashSet<String>,
}

impl Example {
    pub fn new() -> Self {
        Self {
            phone_number: HashSet::new(),
        }
    }
}

impl Default for Example {
    fn default() -> Self {
        Example::new()
    }
}
