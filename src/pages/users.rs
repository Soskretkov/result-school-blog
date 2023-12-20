use super::shared_components::H2;
use crate::components::Icon;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TempUser {
    pub id: String,
    pub login: String,
    pub registered_at: String,
    pub role_id: u8,
}

#[component]
pub fn Users() -> impl IntoView {
    view! {
        <div class="">
            <H2>"Пользователи"</H2>

            <div class="table-header">
                <div>"Логин"</div>
                <div>"Дата регистрации"</div>
                <div>"Роль"</div>
            </div>
            <UsersTableRow></UsersTableRow>
        </div>
    }
}

#[component]
pub fn UsersTableRow() -> impl IntoView {
    let users_list: Vec<TempUser> = vec![TempUser {
        id: "1".to_string(),
        login: "login_test".to_string(),
        registered_at: "некая дата".to_string(),
        role_id: 2,
    }];

    let users = create_rw_signal(users_list);

    // удалить в бд, запросить пользователей, обновить ресурс или сигнал
    let on_click = |_| !unimplemented!();
    let users_view = users
        .with(
            |data| data
                .into_iter()
                .map(|user| {
                    view! {
                        <div class="table-row">
                            <div class="user-data">{user.login.clone()}</div>
                            <Icon on:click=on_click id="fa-trash-o" class="cursor-pointer text-[24px] ml-2.5"/>
                        </div>
                    }
                })
                .collect_view()
        );

    view! {
        {users_view}
    }
}
