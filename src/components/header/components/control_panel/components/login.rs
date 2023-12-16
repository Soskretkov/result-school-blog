use crate::entities::User;
use crate::bff::Server;
use leptos_router::*;
use crate::components::{Button, Icon};
use leptos::*;

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
                        // <span>{rw_user.get().unwrap().login}</span>
                        // <button on:click=on_click class="text-[16px] px-0 py-0 border-none cursor-pointer">
                        //     <Icon id="fa-sign-out"/>
                        // </button>

                        <div class="flex">
                            <div class="text-[18px] font-bold">{rw_user.get().unwrap().login}</div>
                            <button on:click=on_click class="bg-inherit px-0 py-0 border-none cursor-pointer">
                                <Icon id="fa-sign-out" class="text-[18px]"/>
                            </button>
                        </div>
                    }
                    .into_view()
                }

                false => view! {
                    <A href="/login" class="no-underline">
                        <Button>Войти</Button>
                    </A>
                },
            }
        }
    }

    // view! {
    //     <Show
    //         when=move|| {rw_user.with(|data| data.is_none())}
    //         fallback=move || view!{
    //             <div>"Вася"</div>
    //             <button on:click={on_click} class="text-[8px] cursor-pointer">
    //                 <Icon id="fa-sign-out"/>
    //             </button>
    //         }
    //     >
    //         <A href="/login" class="no-underline">
    //             <Button>Войти</Button>
    //         </A>
    //     </Show>
    // }
}
