use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SessionsStore {
    pub data: HashSet<String>,
}

impl SessionsStore {
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.data.iter()
    }

    pub fn del_session(mut self, sess_id: &str) -> Self{
        self.data.remove(sess_id);
        self
    }

    pub fn is_exist(&self, sess_id: &str) -> bool {
        self.data.get(sess_id).is_some()
    }

    pub fn add_rnd_session(&mut self) -> String {
        let sess_id = Uuid::new_v4().to_string();
        self.data.insert(sess_id.clone());
        sess_id
    }
}