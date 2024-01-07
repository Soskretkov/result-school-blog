use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RoleName {
    Administrator,
    Moderator,
    Reader,
}

impl RoleName {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoleName::Administrator => "Администратор",
            RoleName::Moderator => "Модератор",
            RoleName::Reader => "Читатель",
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            RoleName::Administrator => 0,
            RoleName::Moderator => 1,
            RoleName::Reader => 2,
        }
    }

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

    pub fn can_view_users(&self) -> bool {
        match self {
            RoleName::Administrator | RoleName::Moderator => true,
            _ => false,
        }
    }
}

impl fmt::Display for RoleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
