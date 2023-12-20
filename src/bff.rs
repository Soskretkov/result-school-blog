mod db_utils;
mod server;
use crate::utils;
use serde::{Deserialize, Serialize};
pub use server::{Authentic, Server};
use std::collections::HashSet;

const URL: &'static str = "http://localhost:3005";

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Sessions {
    data: HashSet<String>,
}

impl Sessions {
    fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = &String> {
        self.data.iter()
    }

    fn del_session(mut self, session_id: &str) -> Self {
        self.data.remove(session_id);
        self
    }

    fn add_rnd_session(mut self) -> Self {
        let session_id = utils::create_rnd_float64().to_string();
        self.data.insert(session_id);
        self
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    id: String,
    login: String,
    password: String,
    registered_at: String,
    role_id: u8,
    sessions: Sessions,
}
