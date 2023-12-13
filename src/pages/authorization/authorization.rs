use crate::bff::{Authorize, Server};
use crate::components::{Button, Input};
use super::h2::H2;
use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;

#[component]
pub fn Authorization() -> impl IntoView {
    let (authorize, set_authorize) = create_signal::<Option<Authorize>>(None);

    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();

    let on_submit = {
        let async_handler = move |_: ()| {
            let login = login_node_ref.get().unwrap().value();
            let password = password_node_ref.get().unwrap().value();
            async move { Server::authorize(&login, &password).await }
        };

        move |ev: SubmitEvent| {
            ev.prevent_default();
            let server_resp = create_resource(|| (), async_handler);
            // set_authorize.set(server_resp.get());
            todo!()
        }
    };

    view! {
        <div class="flex items-center flex-col">
            <H2>Авторизация</H2>
            <form on:submit = on_submit class="flex flex-col w-[260px]">
                <Input
                    r#type="text"
                    placeholder="Логин..."
                    node_ref = login_node_ref
                />

                <Input
                    r#type="password"
                    placeholder="Пароль..."
                    node_ref = password_node_ref
                />

                <Button
                    r#type="submit"
                    disabled={false}
                >"Авторизоваться"
                </Button>

                <ErrMsg/>

                <A href="/register" class="mt-5 text-[18px] text-center text-black">"Регистрация"</A>

                {move || match authorize.get() {
                    None => {}.into_view(),
                    Some(data) => view! { <div>{data.error}</div> }.into_view()
                }}
            </form>
        </div>
    }
}

#[component]
pub fn ErrMsg() -> impl IntoView {
    view! {
        <div class="bg-red-300 text-[18px] mt-2.5 px-2.5 py-2.5">"Заполните логин"</div>
    }
}
