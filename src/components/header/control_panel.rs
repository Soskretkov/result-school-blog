use crate::components::Icon;
mod login;
use crate::types::Auth;
use leptos::*;
use leptos_router::*;
use login::Login;

#[component]
pub fn ControlPanel(set_auth: WriteSignal<Option<Auth>>) -> impl IntoView {
    let history = leptos::web_sys::window().unwrap().history().unwrap();

    view! {
        <div class="flex flex-col items-end">
            <Login set_auth={set_auth}></Login>

            <div class="w-[95px] mt-2.5 justify-between grid grid-flow-col auto-cols-fr">
                <Icon on:click= move |_| {let _ = history.back();}  id="fa-backward" class="cursor-pointer text-[24px] text-left".to_string()/>

                <A href="/post" class="no-underline text-current">
                    <Icon id="fa-file-text-o" class="text-[24px] text-center".to_string()/>
                </A>

                <A href="/users" class="no-underline text-current">
                    <Icon id="fa-users" class="text-[24px] text-right".to_string()/>
                </A>
            </div>
        </div>
    }
}
