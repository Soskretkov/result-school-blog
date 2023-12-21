use super::components::H2;
use leptos::*;
mod table_body_row;
mod table_row;
use crate::types::outer_api::{Role, User};
use table_body_row::TableBodyRow;
use table_row::TableRow;

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
            <div class="flex items-center">
                <div class="w-[172px] px-2.5">"Логин"</div>
                <div class="w-[213px] px-2.5">"Дата регистрации"</div>
                <div class="w-auto px-2.5">"Роль"</div>
                <div>"удалить"</div>
            </div>
            <TableRow>
                {users
                    .with(
                        |data| data
                            .into_iter()
                            .map(|user| {
                                view!{
                                    <TableBodyRow user={user}/>
                                }
                            })
                            .collect_view()
                    )
                }
            </TableRow>
        </div>
    }
}
