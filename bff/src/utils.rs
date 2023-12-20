use rand::{Rng, thread_rng};

pub fn create_rnd_float64() -> f64 {
    let mut rng = thread_rng(); // Получаем генератор случайных чисел
    let random_float: f64 = rng.gen();
    random_float
}
