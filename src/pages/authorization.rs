use super::components::{FormErrMsg, H2};
use crate::components::{Button, Input};
use crate::types::GlobContext;
use bff::server::{self, Session};
use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;

#[component]
pub fn Authorization(set_session: WriteSignal<Option<Session>>) -> impl IntoView {
    // Пользователь уже авторизован, перенаправляем на главную
    if use_context::<GlobContext>().unwrap().user_info.is_loaded() {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    }

    let (auth_error, set_auth_error) = create_signal::<Option<String>>(None);

    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();

    let on_submit = {
        let authorize = create_action(move |input: &(String, String)| {
            let login = input.0.clone();
            let password = input.1.clone();
            let set_session = set_session.clone();
            let set_auth_error = set_auth_error.clone();

            async move {
                match server::fetch_id_by_login(&login).await {
                    // Some(user_id) => {
                    Some(user_id) => match server::authorize(&user_id, &password).await {
                        Ok(sess_id) => {
                            logging::log!("{}", sess_id);

                            let new_sess = Session {
                                id: sess_id,
                                user_id,
                            };


                            set_session.set(Some(new_sess));

                            // Возврат
                            let _ = leptos::web_sys::window().unwrap().history().unwrap().back();
                        }
                        Err(err_msg) => {
                            set_auth_error.set(Some(err_msg));
                        }
                    },
                    None => {
                        set_auth_error.set(Some("Пользователя не существует".to_string()));
                    }
                };
            }
        });

        move |ev: SubmitEvent| {
            ev.prevent_default();

            let login = login_node_ref.get().unwrap().value();
            let password = password_node_ref.get().unwrap().value();

            authorize.dispatch((login, password));

            // let new_sess = Session {
            //     id: sess_id,
            //     user_id,
            // };

            // set_session.set(Some(new_sess));

            // Возврат
            // let _ = leptos::web_sys::window().unwrap().history().unwrap().back();
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
            </form>
        </div>
    }
}
