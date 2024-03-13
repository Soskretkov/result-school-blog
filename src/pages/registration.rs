use super::components::FormErrMsg;
use crate::components::{Button, Content, Input};
use crate::server::{self, Session};
use crate::types::GlobContext;
use leptos::{ev::SubmitEvent, html::Input, *};

#[component]
pub fn Registration(set_session: WriteSignal<Option<Session>>) -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();

    // Пользователь уже авторизован, перенаправляем
    // Предполагается что невалидные куки отсееваются на старте приложухи
    if glob_ctx.session.with_untracked(Option::is_some) {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
        return {}.into_view();
        // let _ = leptos::web_sys::window().unwrap().history().unwrap().back();
    }

    let (auth_error, set_auth_error) = create_signal::<Option<String>>(None);
    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();
    let passcheck_node_ref = create_node_ref::<Input>();

    let on_submit = {
        let authorize = create_action(move |_: &()| {
            let login = login_node_ref.get().unwrap().value();
            let password = password_node_ref.get().unwrap().value();

            async move {
                match server::register(login.clone(), password.clone()).await {
                    Ok(_) => match server::authorize(&login, &password).await {
                        Ok(sess) => {
                            set_session.set(Some(sess));

                            // Возврат
                            let _ = leptos::web_sys::window().unwrap().history().unwrap().back();
                        }
                        Err(err_msg) => set_auth_error.set(Some(err_msg)),
                    },
                    Err(e) => set_auth_error.set(Some(e)),
                };
            }
        });

        move |ev: SubmitEvent| {
            ev.prevent_default();
            authorize.dispatch(());
        }
    };

    view! {
        <Content header="Регистрация">
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
                    class="w-full"
                    r#type="submit"
                    disabled={false}
                >"Зарегистрироваться"
                </Button>

                <FormErrMsg err_signal=auth_error></FormErrMsg>
            </form>
        </Content>
    }
    .into_view()
}
