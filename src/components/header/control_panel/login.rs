use crate::bff::server;
use crate::components::{Button, Icon};
use crate::types::{GlobContext, UserInfo};
use crate::Session;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login(rw_session: RwSignal<Option<Session>>) -> impl IntoView {
    // выход из учетной записи в гостевое представление
    let logout_action = create_action(move |user_signal: &RwSignal<Option<Session>>| {
        let wrapped_sess = user_signal.get();
        async move {
            if let Some(session) = wrapped_sess {
                if server::logout(&session).await.is_ok() {
                    rw_session.update(|rf| *rf = None);
                }
            }
        }
    });

    let on_click = move |_: ev::MouseEvent| {
        logout_action.dispatch(rw_session);
    };

    view! {
        <Suspense
            fallback=move || {
                logging::log!("Header (logging.rs): Suspense fallback (нет UserInfo)");
                view! {
                    <A href="/login" class="w-full no-underline h-8">
                        <Button>"Войти"</Button>
                    </A>
                }
            }
        >
            {
                // нечто похожее на странице users
                move || match use_context::<GlobContext>().unwrap().user_info.user_data() {
                    Some(info) => {
                        view! {
                            <div class="flex h-8">
                                <div class="mt-[2px] text-[18px] font-bold">{info.login.clone()}</div>
                                <button on:click=on_click class="bg-inherit ml-2.5 px-0 py-0 border-none cursor-pointer">
                                    <Icon id="fa-sign-out" class="text-[24px]"/>
                                </button>
                            </div>
                        }
                        .into_view()
                    }

                    None => view! {
                        <A href="/login" class="w-full no-underline h-8">
                            <Button>Войти</Button>
                        </A>
                    },
                }
            }
        </Suspense>
    }
}
