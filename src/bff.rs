use reqwest;
use serde::Deserialize;


#[derive(Deserialize)]
struct User {
    login: String,
    password: String,
}




pub struct Server;



impl Server {
    pub fn authorize(login: &str, pasword: &str)-> bool {
        unimplemented!()
    }
}