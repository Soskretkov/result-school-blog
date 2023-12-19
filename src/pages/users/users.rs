use crate::components::H2;
use crate::entities::{User, Role};
use leptos::*;

#[component]
pub fn Users() -> impl IntoView {
    // let users_list: Vec<Users> = vec![User{
    //     id: "1",
    //     login: "login_test",
    //     role,
    //     session_id: session.id,
    // }];
    view! {
        <div class="">
            <H2>"Пользователи"</H2>
            
            <div>
                <div>"Логин"</div>
                <div>"Дата регистрации"</div>
                <div>"Роль"</div>
            </div>
        </div>
    }
}
