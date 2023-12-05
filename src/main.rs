use reqwest;
use serde::Deserialize;
mod bff;



#[derive(Deserialize)]
struct User {
    login: String,
    password: String,

}




fn main() {
    println!("Hello, world!");
}
