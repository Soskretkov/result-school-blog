mod tbody_row;
use super::super::components::H2;
use crate::server;
use leptos::*;
use tbody_row::TbodyRow;

#[component]
pub fn Users() -> impl IntoView {
    let users_res = create_resource(
        || (),
        move |_| async move { server::fetch_all_users().await.unwrap() },
    );

    let roles_res = create_resource(
        || (),
        move |_| async move { server::fetch_all_roles().await.unwrap() },
    );

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
                                <For
                                    each=move || users_res.get().unwrap_or(Vec::new())
                                    key=|user| user.id.clone()
                                    children=move |user| {
                                        view! {
                                            <TbodyRow
                                                user={user}
                                                roles_res={roles_res}
                                            />
                                        }
                                    }
                                />
                            </Suspense>
                        }
                    }
                </tbody>
            </table>
        </div>
    }
    .into_view()
}
