use serde::{Deserialize, Serialize};


use crate::utils;
use std::collections::HashSet;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Sessions {
    pub data: HashSet<String>,
}

impl Sessions {
    pub fn new() -> Self {
        Self { data: HashSet::new() }
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.data.iter()
    }

    pub fn del_session(mut self, session_id: &str) -> Self {
        self.data.remove(session_id);
        self
    }

    pub fn add_rnd_session(mut self) -> Self {
        let session_id = utils::create_rnd_float64().to_string();
        self.data.insert(session_id);
        self
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub login: String,
    pub password: String,
    pub registed_at: String,
    pub role_id: u8,
    pub sessions: Sessions,
}
