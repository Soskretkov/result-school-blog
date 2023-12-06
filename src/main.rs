mod bff;
mod shared;
use leptos::*;
mod components;
use components::App;

// #[tokio::main]
fn main() {
    mount_to_body(|| App);
}
