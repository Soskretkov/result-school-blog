use super::components::H2;
use leptos::*;
mod tbody_row;
use crate::bff::server;
use crate::types::{GlobContext, UserInfo};
use tbody_row::TbodyRow;

#[component]
pub fn Users() -> impl IntoView {
    // выяснить клонирование
    let user_info = &mut use_context::<GlobContext>().unwrap().user_info;

    // logging::log!("до переименования: {:?}", user_info.user_data().unwrap().login);
    // logging::log!(
    //     "до обновления: {:?}",
    //     use_context::<GlobContext>()
    //         .unwrap()
    //         .user_info
    //         .user_data()
    //         .unwrap()
    //         .login
    // );
    user_info.refetch();

    // Пользователь не авторизован, перенаправляем на авторизацию
    // if user_info.user_data().is_none() {
    //     let navigate = leptos_router::use_navigate();
    //     navigate("/login", Default::default());
    // }

    // обновить информацию по пользователю
    // user_info.refetch();

    // let users_res = create_resource(
    //     || (),
    //     move |_| async {
    //         // server::fetch_all_users(&use_context::<GlobContext>().unwrap().session.get().unwrap())
    //         //     .await
    //         //     .unwrap()
    //     },
    // );

    // let roles_res = create_resource(
    //     || (),
    //     |_| async {
    //         server::fetch_all_roles(&use_context::<GlobContext>().unwrap().session.get().unwrap())
    //             .await
    //             .unwrap()
    //     },
    // );

    // Ресурсы также предоставляют refetch()метод, который позволяет вручную перезагрузить данные.
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
    }
}
