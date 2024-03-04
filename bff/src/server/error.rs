use std::fmt;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    DbEntryNotFound,
    DbEntryNotUnique,
    Serialization(serde_json::Error),
    Deserialization(serde_json::Error),
    InvalidSession,
    InvalidPassword,
    InternalLogic(String),
    UserPermission,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwest(_) => write!(f, "Не удалось выполнить url-запрос"),
            Error::Serialization(_) => {
                write!(f, "Не удалось сериализовать JSON-формат")
            }
            Error::Deserialization(_) => write!(f, "Не удалось десериализовать JSON-формат"),
            Error::InvalidSession => write!(f, "Недействительная сессия"),
            Error::DbEntryNotFound => write!(f, "Запрашиваемый елемент не найден"),
            Error::DbEntryNotUnique => write!(f, "Нарушение уникальности"),
            Error::InvalidPassword => write!(f, "Неверный пароль"),
            Error::UserPermission => write!(f, "Недостаточно прав на совершение операции"),
            Error::InternalLogic(_) => write!(f, "Логическая ошибка в dll"),
        }
    }
}
