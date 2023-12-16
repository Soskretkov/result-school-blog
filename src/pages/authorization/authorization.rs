use super::h2::H2;
use crate::bff::{Authorize, Server};
use crate::components::{Button, Input};
use crate::entities::User;
use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;


#[component]
pub fn Authorization(rw_user: RwSignal<Option<User>>) -> impl IntoView {
    let (authorize, set_authorize) = create_signal::<Option<Authorize>>(None);

    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();

    let on_submit = {
        let async_handler = move |_: ()| {
            // let set_server = use_context::<WriteSignal<Server>>().unwrap();

            // let authorize = async move {
            //     set_server.update(|serv| {
            //         let auth = serv.authorize(&login, &password);
            //     })
            // };
        };

        move |ev: SubmitEvent| {
            ev.prevent_default();
            // let server_resp = create_resource(|| (), async_handler);
            // set_authorize.set(server_resp.get());

            let login_node = login_node_ref.get().unwrap();
            let password_node = password_node_ref.get().unwrap();

            rw_user.set(Some(User {
                id: "id".to_string(),
                login: login_node.value(),
                role: crate::entities::user::Role::Reader,
                session_id: "session_id".to_string(),
            }));

            // Очистка формы
            login_node.set_value("");
            password_node.set_value("");

            // Возврат
            let _ = leptos::web_sys::window().unwrap().history().unwrap().back();
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
