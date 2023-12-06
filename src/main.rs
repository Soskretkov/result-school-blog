// mod bff;
// mod shared;
use leptos::*;
// use tokio;
mod components;
use components::App;

// #[tokio::main]
fn main() {
    mount_to_body(|| App);
}
