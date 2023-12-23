use super::components::{FormErrMsg, H2};
use crate::bff::server;
use crate::components::{Button, Input};
use crate::Session;
use leptos::{ev::SubmitEvent, html::Input, *};

#[component]
pub fn Registration(rw_session: RwSignal<Option<Session>>) -> impl IntoView {
    let (authentic, set_authentic) = create_signal::<Option<String>>(None);

    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();
    let passcheck_node_ref = create_node_ref::<Input>();

    let on_submit = {
        let async_handler = move |login: String, password: String| {
            // let set_server = use_context::<WriteSignal<Server>>().unwrap();

            let authentic = {
                logging::log!("вызов асинхр. функции");

                // set_server.update(|serv| {
                //     let auth = serv.register(login, password);
                // })
            };
        };

        move |ev: SubmitEvent| {
            ev.prevent_default();
            let login_node = login_node_ref.get().unwrap();
            let password_node = password_node_ref.get().unwrap();
            let passcheck_node = passcheck_node_ref.get().unwrap();

            async_handler(login_node.value(), password_node.value());

            rw_session.set(Some(Session {
                id: "id".to_string(),
                login: login_node.value(),
                registered_at: "некая дата".to_string(),
                role: crate::RoleName::Reader,
                sess_id: "session_id".to_string(),
            }));

            // Очистка формы
            login_node.set_value("");
            password_node.set_value("");
            passcheck_node.set_value("");

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
