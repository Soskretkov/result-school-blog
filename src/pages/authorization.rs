use crate::bff::{Authorize, Server};
use leptos::{ev::SubmitEvent, html::Input, *};
// use crate::shared::types::Role;

#[component]
pub fn Authorization() -> impl IntoView {
    let (error, setError) = create_signal::<Option<String>>(None);

    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let login = login_node_ref.get().unwrap().value();
        let password = password_node_ref.get().unwrap().value();

        // logging::log!("логин {}\nпароль {}", &login, &password);


        let server_resp = create_resource(|| (), move |_| async {
            Server::authorize(&login, "").await
        });
    };

    let error_msg = "сообщение ошибки";

    view! {
        <div class="flex items-center flex-col">
            <h2>Авторизация</h2>
            <form on:submit = on_submit class="flex flex-col">
                <input
                    type="text"
                    placeholder="Логин..."
                    node_ref = login_node_ref
                 />
                <input
                    type="password"
                    placeholder="Пароль..."
                    node_ref = password_node_ref
                />
                <button type="submit">"Войти"</button>
                <div>{error_msg}</div>
            </form>
        </div>
    }
}
