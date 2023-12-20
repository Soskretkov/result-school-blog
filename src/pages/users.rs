use super::components::H2;
use leptos::*;
mod user_row;
use user_row::UserRow;

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
            <UserRow></UserRow>
        </div>
    }
}
