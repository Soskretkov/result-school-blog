use crate::bff::server;
use crate::components::{Button, Icon};
use crate::types::session::Session;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login(rw_user: RwSignal<Option<Session>>) -> impl IntoView {
    let on_click = move |_: ev::MouseEvent| {
        rw_user.with(|wrapped_user| {
            let user = wrapped_user.as_ref().unwrap();

            server::logout(&user.login, &user.session_id);
        });

        rw_user.set(None);
    };

    view! {
        {
            move || match rw_user.with(|data| data.is_some()) {
                true => {
                    view! {
                        <div class="flex h-8">
                            <div class="mt-[2px] text-[18px] font-bold">{rw_user.get().unwrap().login}</div>
                            <button on:click=on_click class="bg-inherit ml-2.5 px-0 py-0 border-none cursor-pointer">
                                <Icon id="fa-sign-out" class="text-[24px]"/>
                            </button>
                        </div>
                    }
                    .into_view()
                }

                false => view! {
                    <A href="/login" class="w-full no-underline h-8">
                        <Button>Войти</Button>
                    </A>
                },
            }
        }
    }
}
