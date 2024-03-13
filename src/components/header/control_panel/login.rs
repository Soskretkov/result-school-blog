use crate::components::{Button, Icon};
use crate::server::{self};
use crate::types::{Auth, GlobContext};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login(set_auth: WriteSignal<Option<Auth>>) -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();

    // выход из учетной записи в гостевое представление
    let on_click = {
        let logout_action = create_action(move |_| async move {
            if use_context::<GlobContext>()
                .unwrap()
                .auth
                .with_untracked(Option::is_some)
            {
                if server::logout().await.is_ok() {
                    set_auth.update(|rf| *rf = None);
                }
            }
        });

        move |_: ev::MouseEvent| {
            logout_action.dispatch(set_auth);
        }
    };

    view! {
        {move || match glob_ctx.auth.get().and_then(|auth| auth.user) {
            Some(user) => view! {
                <div class="flex h-8">
                    <div class="mt-[2px] text-[18px] font-bold">{user.name}</div>
                    <button on:click=on_click class="bg-inherit ml-2.5 px-0 py-0 border-none cursor-pointer">
                        <Icon id="fa-sign-out" class="text-[24px]".to_string()/>
                    </button>
                </div>
            }.into_view(),
            None => view!{<LoginButton/>},
        }}
    }
}

#[component]
pub fn LoginButton() -> impl IntoView {
    view! {
        <A href="/login" class="w-full no-underline h-8">
            <Button class="w-full">"Войти"</Button>
        </A>
    }
}
