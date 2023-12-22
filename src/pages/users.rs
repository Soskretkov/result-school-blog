use super::components::H2;
use leptos::*;
mod table_body;
use crate::types::outer_api::{Role, User};
use table_body::TableBody;

#[component]
pub fn Users() -> impl IntoView {

    let users_list: Vec<User> = vec![User {
        id: "1".to_string(),
        login: "login_test".to_string(),
        registered_at: "некая дата".to_string(),
        role_id: 2,
    }];


    let users = create_rw_signal(users_list);
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
                <TableBody/>
            </table>
        </div>
    }
}
