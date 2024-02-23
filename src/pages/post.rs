mod components;
use crate::components::{Button, Content, Input};
use crate::server::{self};
use components::Comments;
use leptos::*;

#[component]
pub fn Post() -> impl IntoView {
    //   load_post
    view! {
        <div>
            <Comments/>
        </div>
    }
}
