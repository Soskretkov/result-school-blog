use rand::Rng;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Sessions {
    pub store: HashMap<String, String>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn create(&mut self) -> String {
        let mut rng = rand::thread_rng(); // Получаем генератор случайных чисел
        let random_float: f64 = rng.gen();
        let session_id = random_float.to_string();
        self.add(session_id.clone());
        session_id
    }

    pub fn del(&mut self, hash: String) -> Option<String> {
        self.store.remove(&hash)
    }

    fn add(&mut self, hash: String) {
        self.store.insert(hash.clone(), hash);
    }
}
