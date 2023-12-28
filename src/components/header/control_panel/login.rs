use crate::bff::server;
use crate::components::{Button, Icon};
use crate::types::GlobContext;
use crate::Session;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login(rw_session: RwSignal<Option<Session>>) -> impl IntoView {
    let logout_action = create_action(move |session: &RwSignal<Option<Session>>| {
        let sess = session.get();
        async move {
            if let Some(value) = sess {
                let logaut_result = server::logout(&value).await;
                if logaut_result.is_ok() {
                    rw_session.update(|rf| *rf = None);
                }
            }
        }
    });

    let on_click = move |_: ev::MouseEvent| {
        logout_action.dispatch(rw_session);
    };

    let glob_context = use_context::<GlobContext>().unwrap();

    view! {
        // {
        //     // посмотри пример с users
        //     move || match glob_context.user_info.with(Option::is_some) {
        //         true => {
        //             let login = glob_context.user_info.with(|user_info|
        //                 user_info
        //                     .as_ref()
        //                     .and_then(|inner| inner.as_ref())
        //                     .map(|info| info.login.clone())
        //                     .unwrap_or_default()
        //             );

        //             logging::log!("{:#?}", glob_context.user_info.get());

        //             view! {
        //                 <div class="flex h-8">
        //                     <div class="mt-[2px] text-[18px] font-bold">{login}</div>
        //                     <button on:click=on_click class="bg-inherit ml-2.5 px-0 py-0 border-none cursor-pointer">
        //                         <Icon id="fa-sign-out" class="text-[24px]"/>
        //                     </button>
        //                 </div>
        //             }
        //             .into_view()
        //         }

        //         false => view! {
        //             <A href="/login" class="w-full no-underline h-8">
        //                 <Button>Войти</Button>
        //             </A>
        //         },
        //     }
        // }

        <Suspense
            fallback=move || view! {
                <A href="/login" class="w-full no-underline h-8">
                    <Button>Войти</Button>
                </A>
            }
        >
            <div class="flex h-8">
                <div class="mt-[2px] text-[18px] font-bold">{glob_context.user_info.with(|user_info|
                    user_info
                        .as_ref()
                        .and_then(|inner| inner.as_ref())
                        .map(|info| info.login.clone())
                        .unwrap_or_default()
                )}</div>
                <button on:click=on_click class="bg-inherit ml-2.5 px-0 py-0 border-none cursor-pointer">
                    <Icon id="fa-sign-out" class="text-[24px]"/>
                </button>
            </div>
        </Suspense>
    }
}
