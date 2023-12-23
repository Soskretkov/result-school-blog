use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::server::utils;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Sessions {
    pub data: HashSet<String>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.data.iter()
    }

    pub fn del_session(mut self, session_id: &str) -> Self {
        self.data.remove(session_id);
        self
    }

    pub fn is_exist(&self, session_id: &str) -> bool {
        self.data.get(session_id).is_some()
    }

    pub fn add_rnd_session(mut self) -> Self {
        let session_id = utils::create_rnd_float64().to_string();
        self.data.insert(session_id);
        self
    }
}