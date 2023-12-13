use crate::components::{Icon, Button};
use leptos::*;
use leptos_router::*;

#[component]
pub fn ControlPanel() -> impl IntoView {
    let history = leptos::web_sys::window().unwrap().history().unwrap();

    view! {
        <div>
            <A href="/login" class="no-underline">
                <Button>Войти</Button>
            </A>
            <div class="flex justify-end">
                <button on:click= move |_| history.back().unwrap() class="text-[24px] mt-[10px] cursor-pointer">
                    <Icon id="fa-backward"/>
                </button>

                <A href="/post" class="no-underline text-current">
                    <Icon id="fa-file-text-o" class="text-[24px] mt-[10px] ml-[17px]"/>
                </A>
                <A href="/users" class="no-underline text-current">
                    <Icon id="fa-users" class="text-[24px] mt-[10px] ml-[17px]"/>
                </A>
            </div>
        </div>
    }
}
