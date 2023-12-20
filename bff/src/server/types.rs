use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
// клиентская сторона не конфигурирует Authorize, это ответ сервера
pub struct Authentic {
    pub error: Option<String>,
    pub res: Option<String>,
}
