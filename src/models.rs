use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: Option<usize>,
    pub start: String,
    pub end: String,
}

impl Route {
    pub fn new(id: usize, start: String, end: String) -> Self {
        Route { id: Some(id), start, end }
    }
}