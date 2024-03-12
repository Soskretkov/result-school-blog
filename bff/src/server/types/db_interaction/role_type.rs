use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum RoleType {
    Administrator,
    Moderator,
    Reader,
}

impl RoleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoleType::Administrator => "Администратор",
            RoleType::Moderator => "Модератор",
            RoleType::Reader => "Читатель",
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            RoleType::Administrator => 0,
            RoleType::Moderator => 1,
            RoleType::Reader => 2,
        }
    }

    pub fn from_u8(id: u8) -> Option<Self> {
        match id {
            0 => Some(RoleType::Administrator),
            1 => Some(RoleType::Moderator),
            2 => Some(RoleType::Reader),
            _ => None,
        }
    }

    // использование: бек - сверяет право на извлечение, фронт - возможность попасть на страницу
    pub fn can_view_users(&self) -> bool {
        match self {
            RoleType::Administrator => true,
            _ => false,
        }
    }

    pub fn can_view_roles(&self) -> bool {
        match self {
            _ => true, // любой у кого есть сессия
        }
    }

    pub fn can_update_roles(&self) -> bool {
        match self {
            RoleType::Administrator => true,
            _ => false,
        }
    }

    pub fn can_remove_users(&self) -> bool {
        match self {
            RoleType::Administrator => true,
            _ => false,
        }
    }

    pub fn can_remove_comment(&self) -> bool {
        match self {
            RoleType::Administrator | RoleType::Moderator => true,
            _ => false,
        }
    }
}

impl fmt::Display for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for RoleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_u8().to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RoleType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Десериализация строки
        let s = String::deserialize(deserializer)?;
        // Попытка преобразовать строку в u8
        let id = s.parse::<u8>().map_err(serde::de::Error::custom)?;
        // Получение RoleType из u8
        RoleType::from_u8(id).ok_or_else(|| serde::de::Error::custom("Invalid role id"))
    }
}
