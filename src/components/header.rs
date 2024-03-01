mod control_panel;
mod logo;
use crate::server::Session;
use control_panel::ControlPanel;
use leptos::*;
use logo::Logo;

#[component]
pub fn Header(set_session: WriteSignal<Option<Session>>) -> impl IntoView {
    view! {
        // высоту сделал меньше чем в оригинальной верстке в видео (плохо смотрелось на моем ноуте)
        // в оригинале h-[120px] и в дочернем элементе py-[20px]
        <header class="flex justify-between fixed top-0 w-[1000px] h-[100px] py-[10px] px-10 bg-white shadow-md z-10">
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
