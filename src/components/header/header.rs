use leptos::*;
use super::components::Logo;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-white h-[120px] w-[1000px] py-5 px-10 shadow-md fixed top-0">
            <Logo/>
        </header>
    }
}