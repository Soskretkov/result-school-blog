use crate::components::Icon;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <A href="/" class="flex -mt-[21px] no-underline text-current">
            <Icon id="fa-code" class="text-[70px] mr-[10px]".to_string()/>
            <div class="">
                <div class="text-[48px] font-semibold leading-[48px] mt-4">"Блог"</div>
                <div class="text-[18px] font-bold">"веб-разработка"</div>
            </div>
        </A>
    }
}
