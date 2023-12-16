use crate::components::Icon;
use crate::entities::User;
use super::components::Login;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ControlPanel(rw_user: RwSignal<Option<User>>) -> impl IntoView {
    let history = leptos::web_sys::window().unwrap().history().unwrap();

    view! {
        <div class="flex flex-col items-end">
            <Login rw_user={rw_user}></Login>

            <div class="w-[95px] mt-2.5 justify-between grid grid-flow-col auto-cols-fr">
                <Icon on:click= move |_| history.back().unwrap()  id="fa-backward" class="cursor-pointer text-[24px] text-left"/>

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

