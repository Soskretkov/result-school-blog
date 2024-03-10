mod table_body;
use crate::components::{Content, PageErrMsg};
use crate::server::{self};
use crate::types::GlobContext;
use leptos::*;
use table_body::TableBody;

#[component]
pub fn UsersContent() -> impl IntoView {
    if !can_access() {
        return view! { <PageErrMsg>"Недостаточно прав для просмотра страницы"</PageErrMsg>};
    }

    let users_res = create_resource(
        || (),
        move |_| async move { server::fetch_all_users().await },
    );

    // внешний view чтобы отслеживался .get()
    view! {
        {move || users_res.get().map(|wr_users|
            match wr_users {
                Ok(users_vec) => {
                    view! {
                        <Content header="Пользователи">
                            <table class="w-[570px] mx-auto text-[18px]">
                                <thead  class="">
                                    <tr class="flex items-start text-left [&>*:not(:last-child)]:px-2.5">
                                        <th class="w-[170px] font-normal">"Имя"</th>
                                        <th class="w-[170px] font-normal">"Дата регистрации"</th>
                                        <th class="w-auto font-normal">"Роль"</th>
                                        <th class="w-auto font-normal"></th>
                                    </tr>
                                </thead>
                                <TableBody users_vec=users_vec />
                            </table>
                        </Content>
                    }.into_view()
                }
                Err(e) => {
                    view! {<PageErrMsg>{e.clone()}</PageErrMsg>}
                },
            }
        )}
    }
    .into_view()
}

fn can_access() -> bool {
    match use_context::<GlobContext>()
        .unwrap()
        .user_resource
        .get()
        .and_then(|wr_user| wr_user)
    {
        Some(user) => {
            let role = user.role_id;
            role.can_view_users() && role.can_view_roles()
        }
        None => false,
    }
}
