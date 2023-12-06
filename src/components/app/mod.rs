use leptos::*;
use crate::bff;

#[component]
pub fn App() -> impl IntoView {
    let users = create_resource(|| (), |_| async move {
        bff::db_utils::get_users().await
    });

    view! {
        <div>
            {
                // Отображаем данные или сообщение о загрузке
                match users.get() {
                    Some(data) => view! { <p>{format!("Users: {:?}", data)}</p> },
                    None => view! { <p>{"Loading users..."}</p> }
                }
            }
        </div>
    }
}

#[component]
pub fn Load() -> impl IntoView {
    let users = create_resource(|| (), |_| async move {
        bff::db_utils::get_users().await
    });

    view! {
        <div>
            {
                // Отображаем данные или сообщение о загрузке
                match users.get() {
                    Some(data) => view! { <p>{format!("Users: {:?}", data)}</p> },
                    None => view! { <p>{"Loading users..."}</p> }
                }
            }
        </div>
    }
}
