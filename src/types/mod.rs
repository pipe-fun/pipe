use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}