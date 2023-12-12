use crate::bff::{Authorize, Server};
use leptos::{ev::SubmitEvent, html::Input, *};
// use crate::shared::types::Role;

#[component]
pub fn Authorization() -> impl IntoView {
    let (authorize, set_authorize) = create_signal::<Option<Authorize>>(None);

    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();

    let async_handler = move |_: () | {
        let login = login_node_ref.get().unwrap().value();
        let password = password_node_ref.get().unwrap().value();
        async move {
            Server::authorize(&login, &password).await
        }
    };

    let on_submit = {
        move |ev: SubmitEvent| {
            ev.prevent_default();
            let server_resp = create_resource(|| (), async_handler);
            set_authorize.set(server_resp.get());
        }
    };


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
                {move || match authorize.get() {
                    None => {}.into_view(),
                    Some(data) => view! { <div>{data.error}</div> }.into_view()
                }}
            </form>
        </div>
    }
}
