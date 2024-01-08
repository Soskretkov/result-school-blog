use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
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

    // использование: бек - сверяет право на извлечение, фронт - возможность попасть на страницу
    pub fn can_view_users(&self) -> bool {
        match self {
            RoleName::Administrator | RoleName::Moderator => true,
            _ => false,
        }
    }

    pub fn can_view_roles(&self) -> bool {
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

impl Serialize for RoleName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_u8().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RoleName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /* Десериализация числа: в первую очередь, deserializer берет данные из JSON и преобразует их в число типа u8. В этом процессе он еще не знает ничего о RoleName. Он просто выводит тип раст u8. */
        let id = u8::deserialize(deserializer)?;
        RoleName::from_id(id).ok_or_else(|| serde::de::Error::custom("Invalid role id"))
    }
}
