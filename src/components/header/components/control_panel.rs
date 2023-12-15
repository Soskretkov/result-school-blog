use crate::bff::Server;
use crate::components::{Button, Icon};
use crate::entities::User;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ControlPanel(rw_user: RwSignal<Option<User>>) -> impl IntoView {
    let history = leptos::web_sys::window().unwrap().history().unwrap();

    view! {
        <div>
            <Login rw_user={rw_user}></Login>
            <div class="flex justify-end">
                <button on:click= move |_| history.back().unwrap() class="text-[24px] mt-[10px] cursor-pointer">
                    <Icon id="fa-backward"/>
                </button>

                <A href="/post" class="no-underline text-current">
                    <Icon id="fa-file-text-o" class="text-[24px] mt-[10px] ml-[17px]"/>
                </A>
                <A href="/users" class="no-underline text-current">
                    <Icon id="fa-users" class="text-[24px] mt-[10px] ml-[17px]"/>
                </A>
            </div>
        </div>
    }
}

#[component]
pub fn Login(rw_user: RwSignal<Option<User>>) -> impl IntoView {
    let on_click = move |_| {
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
                        <div>{rw_user.get().unwrap().login}</div>
                        <button on:click=on_click class="text-[8px] cursor-pointer">
                            <Icon id="fa-sign-out"/>
                        </button>
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
