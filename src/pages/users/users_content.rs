mod tbody_row;
use crate::components::{Content, PageErrMsg};
use crate::server;
use crate::types::GlobContext;
use leptos::*;
use tbody_row::TbodyRow;

#[component]
pub fn UsersContent() -> impl IntoView {
    if !can_access() {
        return view! { <PageErrMsg err_msg={"Недостаточно прав для просмотра страницы".to_string()}/> };
    }

    let users_res = create_resource(
        || (),
        move |_| async move {
            logging::log!("users.rs: async все пользователи");
            server::fetch_all_users().await
        },
    );

    // внешний view чтобы отслеживался .get()
    view! {
        {move || users_res.get().map(|wr_users|
            match wr_users {
                Ok(users_vec) => {
                    view! {
                        <Content header="Пользователи">
                            <table class="w-[570px] mx-auto text-[18px]">
                                <thead>
                                    <tr class="flex items-start">
                                        <th class="w-[170px] px-2.5">"Логин"</th>
                                        <th class="w-[170px] px-2.5">"Дата регистрации"</th>
                                        <th class="w-[195px] px-2.5">"Роль"</th>
                                        <th class="w-auto"></th>
                                    </tr>
                                </thead>
                                <tbody class="[&>*>*:not(:last-child)]:border-t [&>*>*]:border-l [&>*:last-child>*:not(:last-child)]:border-b [&>*>*]:border-black">{
                                    users_vec
                                        .into_iter()
                                        .map(|user| {
                                            view! { <TbodyRow user={user}/> }
                                        })
                                        .collect_view()
                                }</tbody>
                            </table>
                        </Content>
                    }.into_view()
                }
                Err(e) => view! { <PageErrMsg err_msg={e}/> },
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
