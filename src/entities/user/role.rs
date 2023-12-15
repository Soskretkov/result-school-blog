use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Role {
    Administrator,
    Moderator,
    Reader,
}

impl Role {
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(Role::Administrator),
            1 => Some(Role::Moderator),
            2 => Some(Role::Reader),
            _ => None,
        }
    }
    pub fn can_remove_comment(&self) -> bool {
        match self {
            Role::Administrator | Role::Moderator => true,
            _ => false,
        }
    }
}