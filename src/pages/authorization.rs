use super::components::{FormErrMsg, H2};
use crate::bff::{Authentic, bff_procs};
use crate::components::{Button, Input};
use crate::types::session::{Session, RoleName};
use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;

#[component]
pub fn Authorization(rw_session: RwSignal<Option<Session>>) -> impl IntoView {
    let (authentic, set_authentic) = create_signal::<Option<Authentic>>(None);

    let login_node_ref = create_node_ref::<Input>();
    let password_node_ref = create_node_ref::<Input>();

    let on_submit = {
        let async_handler = move |_: ()| {
            // let set_server = use_context::<WriteSignal<Server>>().unwrap();

            // let authentic = async move {
            //     set_server.update(|serv| {
            //         let auth = serv.authorize(&login, &password);
            //     })
            // };
        };

        move |ev: SubmitEvent| {
            ev.prevent_default();
            // let server_resp = create_resource(|| (), async_handler);
            // set_authentic.set(server_resp.get());

            let login_node = login_node_ref.get().unwrap();
            let password_node = password_node_ref.get().unwrap();

            rw_session.set(Some(Session {
                id: "id".to_string(),
                login: login_node.value(),
                registered_at: "некая дата".to_string(),
                role: RoleName::Reader,
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
                >"Войти"
                </Button>

                // <FormErrMsg/>

                <A href="/register" class="mt-5 text-[18px] text-center text-black">"Регистрация"</A>

                {move || match authentic.get() {
                    None => {}.into_view(),
                    Some(data) => view! { <div>{data.error}</div> }.into_view()
                }}
            </form>
        </div>
    }
}
