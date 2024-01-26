use crate::components::Icon;
use crate::types::AuthedUser;
mod login;
use leptos::*;
use leptos_router::*;
use login::Login;

#[component]
pub fn ControlPanel(set_authed_user: WriteSignal<Option<AuthedUser>>) -> impl IntoView {
    let history = leptos::web_sys::window().unwrap().history().unwrap();

    view! {
        <div class="flex flex-col items-end">
            <Login set_authed_user={set_authed_user}></Login>

            <div class="w-[95px] mt-2.5 justify-between grid grid-flow-col auto-cols-fr">
                <Icon on:click= move |_| {let _ = history.back();}  id="fa-backward" class="cursor-pointer text-[24px] text-left"/>

                <A href="/post" class="no-underline text-current">
                    <Icon id="mx-[5px] fa-file-text-o" class="text-[24px] text-center"/>
                </A>

                <A href="/users" class="no-underline text-current">
                    <Icon id="fa-users" class="text-[24px] text-right"/>
                </A>
            </div>
        </div>
    }
}
