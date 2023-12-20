use crate::components::Icon;
use crate::types::outer_api::{Role, User};
use leptos::*;

#[component]
pub fn UserRow() -> impl IntoView {
    let users_list: Vec<User> = vec![User {
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
                            <div class="user-data">
                                <div>{user.login.clone()}</div>
                                <div>{user.registered_at.clone()}</div>
                                <div>
                                    <RoleSelect user_role_id={user.role_id}></RoleSelect>
                                    <Icon on:click=on_click id="fa-floppy-o" class="cursor-pointer text-[24px] ml-2.5"/>
                                </div>
                            </div>
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

#[component]
pub fn RoleSelect(user_role_id: u8) -> impl IntoView {
    view! {
        <select value={user_role_id}>
            <option value="">"Имя роли"</option>
        </select>
    }
}
