use crate::components::{Button, Icon};
use crate::server::Session;
use crate::server::{self};
use crate::types::GlobContext;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login(set_session: WriteSignal<Option<Session>>) -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();

    // выход из учетной записи в гостевое представление
    let logout_action = create_action(move |_| async move {
        if use_context::<GlobContext>()
            .unwrap()
            .session
            .with_untracked(Option::is_some)
        {
            if server::logout().await.is_ok() {
                set_session.update(|rf| *rf = None);
            }
        }
    });

    let on_click = move |_: ev::MouseEvent| {
        logout_action.dispatch(set_session);
    };

    // похожая логика в PageGuard
    // внешний view чтобы отслеживался .with()
    view! {
        {move || match glob_ctx.session.with(Option::is_some) {
            true => {
                view! {
                    <Transition
                        fallback=move || {
                            logging::log!("Header (logging.rs): Suspense fallback (нет UserResource)");
                            view!{<LoginButton/>}
                        }
                    >{
                        // в учебном примере тоже вкладывается в замыкание
                        move || glob_ctx.user_resource.get().and_then(|wr_user| wr_user.map(|user| {
                            view! {
                                <div class="flex h-8">
                                    <div class="mt-[2px] text-[18px] font-bold">{user.login.clone()}</div>
                                    <button on:click=on_click class="bg-inherit ml-2.5 px-0 py-0 border-none cursor-pointer">
                                        <Icon id="fa-sign-out" class="text-[24px]"/>
                                    </button>
                                </div>
                            }
                        }))
                    }</Transition>
                }
            }
            false => view!{<LoginButton/>},
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
