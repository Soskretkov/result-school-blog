use super::components::Logo;
use super::components::ControlPanel;
use crate::entities::User;
use leptos::*;

#[component]
pub fn Header(rw_user: RwSignal<Option<User>>) -> impl IntoView {
    view! {
        <header class="flex justify-between bg-white h-[120px] w-[1000px] py-5 px-10 shadow-md fixed top-0">
            <Logo/>
            <Description/>
            <ControlPanel rw_user={rw_user}/>
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
