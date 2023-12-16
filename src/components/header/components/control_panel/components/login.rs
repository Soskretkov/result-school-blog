use crate::bff::Server;
use crate::components::{Button, Icon};
use crate::entities::User;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login(rw_user: RwSignal<Option<User>>) -> impl IntoView {
    let on_click = move |_: ev::MouseEvent| {
        rw_user.with(|user| {
            let session_id = &(user.as_ref().unwrap().session_id);

            use_context::<WriteSignal<Server>>().unwrap().update(|sv| {
                sv.logout(session_id);
            });
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
