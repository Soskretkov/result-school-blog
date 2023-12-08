use super::components::Logo;
use super::components::ControlPanel;
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex justify-between bg-white h-[120px] w-[1000px] py-5 px-10 shadow-md fixed top-0">
            <Logo/>
            <Description/>
            <ControlPanel/>
        </header>
    }
}

#[component]
fn Description() -> impl IntoView {
    view! { 
        <div class="italic">
            Веб-технологии
            <br/>
            Написание кода
            <br/>
            Разбор ошибок
        </div>
    }
}
