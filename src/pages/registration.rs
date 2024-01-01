use super::components::{FormErrMsg, H2};
use crate::components::{Button, Input};
use crate::types::{RoleName, GlobContext};
use leptos::{ev::SubmitEvent, html::Input, *};
use bff::server::Session;


#[component]
pub fn Registration(set_session: WriteSignal<Option<Session>>) -> impl IntoView {
    // Пользователь уже авторизован, перенаправляем на главную
    if use_context::<GlobContext>().unwrap().user_info.is_loaded() {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    }

    let (authentic, set_authentic) = create_signal::<Option<String>>(None);


    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();
    let passcheck_node_ref = create_node_ref::<Input>();

    let on_submit = {
        let async_handler = move |login: String, password: String| {
            logging::log!("вызов асинхр. функции: логин - {login}, пароль - {password}");
        };

        move |ev: SubmitEvent| {
            ev.prevent_default();
            let login_node = login_node_ref.get().unwrap();
            let password_node = password_node_ref.get().unwrap();

            async_handler(login_node.value(), password_node.value());

            // заблокировать поток пока не получу id (отправить логин и пароль на регистрацию)
            // в случае ошибки установить сигнал ошибки в Some и прерваться
            let user_id = "10".to_string();

            // заблокировать поток пока не получу sess_id (отправить id и пароль на авторизацию)
            // в случае ошибки установить сигнал ошибки в Some и прерваться
            let sess_id = "777".to_string();

            let session = Session {
                id: sess_id,
                user_id,
            };
            
            set_session.set(Some(session));

            // установить сигнал ошибки в None, если он Some
            ();

            // Возврат
            let _ = leptos::web_sys::window().unwrap().history().unwrap().back();
        }
    };

    view! {
        <div class="flex items-center flex-col">
            <H2>Регистрация</H2>
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

                <Input
                    r#type="password"
                    placeholder="Проверка пароля..."
                    node_ref = passcheck_node_ref
                />

                <Button
                    r#type="submit"
                    disabled={false}
                >"Зарегистрироваться"
                </Button>

                // <FormErrMsg/>

                // {move || match authentic.get() {
                //     None => {}.into_view(),
                //     Some(data) => view! { <div>{data.error}</div> }.into_view()
                // }}
            </form>
        </div>
    }
}
