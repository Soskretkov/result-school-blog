use super::components::{FormErrMsg, H2};
use crate::components::{Button, Input};
use crate::types::{GlobContext, UserInfo};

use bff::server::Session;
use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;

#[component]
pub fn Authorization(rw_session: RwSignal<Option<Session>>) -> impl IntoView {
    // Пользователь уже авторизован, перенаправляем на главную
    if use_context::<GlobContext>().unwrap().user_info.is_loaded() {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    }

    // logging::log!("{:?}", use_location().pathname.get());

    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();

    let on_submit = {
        // let async_handler = move |_: ()| {
        //     let set_server = use_context::<WriteSignal<Server>>().unwrap();

        //     let authentic = async move {
        //         set_server.update(|serv| {
        //             let auth = serv.authorize(&login, &password);
        //         })
        //     };
        // };

        move |ev: SubmitEvent| {
            ev.prevent_default();

            let login_node = login_node_ref.get().unwrap();
            let password_node = password_node_ref.get().unwrap();

            // заблокировать поток пока не получу id по логину
            // в случае ошибки установить сигнал ошибки в Some и прерваться
            let user_id = "001".to_string();

            // заблокировать поток пока не получу sess_id (отправить id и пароль на регистрацию)
            // в случае ошибки установить сигнал ошибки в Some и прерваться
            let sess_id = "777".to_string();

            let session = Session {
                id: sess_id,
                user_id,
            };

            rw_session.set(Some(session));

            // установить сигнал ошибки в None, если он Some
            ();

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
                >"Войти"
                </Button>

                // <FormErrMsg/>

                <A href="/register" class="mt-5 text-[18px] text-center text-black">"Регистрация"</A>

                // {move || match authentic.get() {
                //     None => {}.into_view(),
                //     Some(data) => view! { <div>{data.error}</div> }.into_view()
                // }}
            </form>
        </div>
    }
}
