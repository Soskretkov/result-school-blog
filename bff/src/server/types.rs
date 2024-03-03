pub mod db_interaction;
pub mod export;
mod sessions_store;
use serde::{Deserialize, Serialize};

pub use sessions_store::SessionsStore;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}
