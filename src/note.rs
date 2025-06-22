use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub created: u64,
    pub edited: u64,
}

impl Note {
    pub fn new(title: &str) -> Self {
        let now = Self::now_ts();
        Self {
            title: title.to_owned(),
            content: String::new(),
            created: now,
            edited: now,
        }
    }

    pub fn now_ts() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}