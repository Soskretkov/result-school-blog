use super::components::{FormErrMsg, H2};
use crate::bff::{Authentic, Server};
use crate::components::{Button, Input};
use crate::entities::User;
use leptos::{ev::SubmitEvent, html::Input, *};

#[component]
pub fn Registration(rw_user: RwSignal<Option<User>>) -> impl IntoView {
    let (authentic, set_authentic) = create_signal::<Option<Authentic>>(None);

    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();
    let passcheck_node_ref = create_node_ref::<Input>();

    let on_submit = {
        let async_handler = move |login: String, password: String| {
            let set_server = use_context::<WriteSignal<Server>>().unwrap();

            let authentic =  {
                logging::log!("вызов асинхр. функции");

                set_server.update(|serv| {
                    let auth = serv.register(login, password);
                })
            };
        };

        move |ev: SubmitEvent| {
            ev.prevent_default();
            let login_node = login_node_ref.get().unwrap();
            let password_node = password_node_ref.get().unwrap();
            let passcheck_node = passcheck_node_ref.get().unwrap();

            async_handler(login_node.value(), password_node.value());

            rw_user.set(Some(User {
                id: "id".to_string(),
                login: login_node.value(),
                role: crate::entities::user::Role::Reader,
                session_id: "session_id".to_string(),
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

                <FormErrMsg/>

                {move || match authentic.get() {
                    None => {}.into_view(),
                    Some(data) => view! { <div>{data.error}</div> }.into_view()
                }}
            </form>
        </div>
    }
}
