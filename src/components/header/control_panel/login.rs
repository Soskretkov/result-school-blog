use crate::components::{Button, Icon};
use crate::server::{self};
use crate::types::{Auth, GlobContext};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login(set_authed_user: WriteSignal<Option<Auth>>) -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();

    // выход из учетной записи в гостевое представление
    let on_click = move |_: ev::MouseEvent| {
        create_action(move |_| async move {
            if glob_ctx.auth.with_untracked(Option::is_some) {
                if server::logout().await.is_ok() {
                    set_authed_user.update(|rf| *rf = None);
                }
            }
        })
        .dispatch(());
    };

    // похожая логика в PageGuard
    // внешний view чтобы отслеживался .get()
    view! {
        {move || match glob_ctx.auth.get() {
            Some(auth) => {
                view! {
                    <Transition
                        fallback=move || {
                            logging::log!("Header (logging.rs): Suspense fallback (нет UserResource)");
                            view!{<LoginButton/>}
                        }
                    >{
                        // в учебном примере тоже вкладывается в замыкание
                        move || auth.user_resource.get().map(|user| {
                            view! {
                                <div class="flex h-8">
                                    <div class="mt-[2px] text-[18px] font-bold">{user.login.clone()}</div>
                                    <button on:click=on_click class="bg-inherit ml-2.5 px-0 py-0 border-none cursor-pointer">
                                        <Icon id="fa-sign-out" class="text-[24px]"/>
                                    </button>
                                </div>
                            }
                        })
                    }</Transition>
                }
            }
            None => view!{<LoginButton/>},
        }}
    }
}

#[component]
pub fn LoginButton() -> impl IntoView {
    view! {
        <A href="/login" class="w-full no-underline h-8">
            <Button>"Войти"</Button>
        </A>
    }
}
