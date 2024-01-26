mod tbody_row;
use super::super::components::{PageErrMsg, H2};
use crate::server;
use crate::types::GlobContext;
use leptos::*;
use tbody_row::TbodyRow;

#[component]
pub fn UsersContent() -> impl IntoView {
    logging::log!("users.rs: монтируется");
    let glob_ctx = use_context::<GlobContext>().unwrap();

    let users_res = create_resource(
        || (),
        move |_| async move {
            logging::log!("users.rs: async все пользователи");
            server::fetch_all_users().await
        },
    );

    view! {
        <Suspense
            // если ресурсы грузятся ничего не показываем
            fallback=|| ()
        >
            <Show
                when=move || {
                    // glob_ctx.user_info.track();
                    can_access()
                    &&
                    users_res.with(|x| x.as_ref().map(Result::is_ok)).unwrap_or(false)
                    &&
                    glob_ctx.roles.value().with(|x| x.as_ref().map(Result::is_ok)).unwrap_or(false)
                }
                fallback=move || {
                    let err_msg = match (users_res.get(), glob_ctx.roles.value().get()) {
                        (Some(Err(e)), _) => e,
                        (_, Some(Err(e))) => e,
                        _ => "".to_string(),
                    };

                    view! {
                        <PageErrMsg err_msg={err_msg}/>
                    }
                }
            >
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
                            <For
                                each=move || users_res.get().unwrap().unwrap_or(Vec::new())
                                key=|user| user.id.clone()
                                children=move |user| {
                                    view! {
                                        <TbodyRow
                                            user={user}
                                        />
                                    }
                                }
                            />
                        </tbody>
                    </table>
                </div>
            </Show>
        </Suspense>
    }
}

fn can_access() -> bool {
    match use_context::<GlobContext>()
        .unwrap()
        .auth
        .get_untracked()
        .unwrap()
        .user_resource
        .get_data()
    {
        Some(user) => {
            let role = user.role_id;
            role.can_view_users() && role.can_view_roles()
        }
        None => false,
    }
}
