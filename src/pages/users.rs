use super::components::H2;
use crate::components::page_guard::Protected;
use leptos::*;
mod tbody_row;
use crate::bff::server;
use crate::types::{GlobContext, UserInfo};
use crate::utils::isSyncServerClientRoles;
use tbody_row::TbodyRow;

pub struct UsersPage;

impl Protected for UsersPage {
    fn view(&self) -> View {
        view! {
            <Users/>
        }
    }

    fn can_access(&self) -> bool {
        let (can_access, set_access) = create_signal(false);
        let glob_ctx = use_context::<GlobContext>().unwrap().user_info.is_loaded();

        let action = create_action(move |_: &()| async move {
            let is_sync_roles = isSyncServerClientRoles().await;
            set_access.set(is_sync_roles);
        });

        //синхронизация
        //роль
        true
    }
}

#[component]
fn Users() -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();

    let users_res = create_resource(
        || (),
        move |_| {
            let sess = glob_ctx.session.get().unwrap();
            async move { server::fetch_all_users(&sess).await.unwrap() }
        },
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
