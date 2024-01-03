use super::components::H2;
use leptos::*;
mod tbody_row;
use crate::bff::server;
use crate::types::{GlobContext, UserInfo};
use tbody_row::TbodyRow;

#[component]
pub fn Users() -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();

    // Пользователь не авторизован, перенаправляем откуда пришел
    if glob_ctx.user_info.user_data().is_none() {
        logging::log!("тут");
        // let _ = leptos::web_sys::window().unwrap().history().unwrap().back();
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    }

    logging::log!("тут не должно быть");

    // let roles_res = create_resource(
    //     || (),
    //     move |_| {
    //         let sess = glob_ctx.session.get().unwrap();
    //         async move { server::fetch_all_roles(&sess).await.unwrap() }
    //     },
    // );

    // let users_res = create_resource(
    //     || (),
    //     move |_| {
    //         let sess = glob_ctx.session.get().unwrap();
    //         async move { server::fetch_all_users(&sess).await.unwrap() }
    //     },
    // );

    let user_info = glob_ctx.user_info;

    view! {
        <div class="flex items-center flex-col w-[570px] mx-auto">
            <H2>"Пользователи"</H2>
            <table>
                <thead>
                    <tr class="flex items-center">
                        <th class="w-[172px] px-2.5">"Логин"</th>
                        <th class="w-[213px] px-2.5">"Дата регистрации"</th>
                        <th class="w-[150px] px-2.5">"Роль"</th>
                        <th class="w-auto"></th>
                    </tr>
                </thead>
                <tbody>
                {move ||
                    view! {
                        <Suspense
                            fallback=move || view! { <p class="text-center">"Loading..."</p> }
                        >
                            // debug
                            <div>{|| use_context::<GlobContext>().unwrap().user_info.user_data().unwrap().login}</div>

                            // <For
                            //     each=move || users_res.get().unwrap_or(Vec::new())
                            //     key=|user| user.id.clone()
                            //     children=move |user| {
                            //         view! {
                            //             <TbodyRow
                            //                 user={user}
                            //                 roles={roles_res}
                            //             />
                            //         }
                            //     }
                            // />
                        </Suspense>
                    }
                }
            </tbody>
            </table>
        </div>
    }.into_view()
}
