use super::components::H2;
use leptos::*;
mod tbody_row;
use tbody_row::TbodyRow;
use crate::bff::server;


#[component]
pub fn Users() -> impl IntoView {
    let users_res = create_resource(|| (), |_| async { server::fetch_all_users("", "").await.unwrap() });
    let roles_res = create_resource(|| (), |_| async { server::fetch_all_roles("", "").await.unwrap() });

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
                {move || {
                    // match users_res.get() {
                    //     None => ().into_view(),
                    //     Some(vec) => {
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
                                                    roles={roles_res}
                                                />
                                            }
                                        }
                                    />
                                </Suspense>
                            }
                        }
                    }
                // }}
            </tbody>
            </table>
        </div>
    }
}
