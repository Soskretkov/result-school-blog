use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RoleName {
    Administrator,
    Moderator,
    Reader,
}

impl RoleName {
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(RoleName::Administrator),
            1 => Some(RoleName::Moderator),
            2 => Some(RoleName::Reader),
            _ => None,
        }
    }
    pub fn can_remove_comment(&self) -> bool {
        match self {
            RoleName::Administrator | RoleName::Moderator => true,
            _ => false,
        }
    }
}