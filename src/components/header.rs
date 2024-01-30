mod control_panel;
mod logo;
use crate::server::Session;
use control_panel::ControlPanel;
use leptos::*;
use logo::Logo;

#[component]
pub fn Header(set_session: WriteSignal<Option<Session>>) -> impl IntoView {
    view! {
        <header class="flex justify-between bg-white h-[120px] w-[1000px] py-5 px-10 shadow-md fixed top-0">
            <Logo/>
            <Description/>
            <ControlPanel set_session={set_session}/>
        </header>
    }
}

#[component]
fn Description() -> impl IntoView {
    view! {
        <div class="italic">
            "Веб-технологии"
            <br/>
            "Написание кода"
            <br/>
            "Разбор ошибок"
        </div>
    }
}
