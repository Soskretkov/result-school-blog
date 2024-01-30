use leptos::*;
mod components;
mod pages;
mod server;
mod types;
mod utils;
use bff::server::{self as bff_server, User};
use components::{Footer, Header};
use leptos_router::*;
use pages::{Authorization, Registration, Users};
use server::Session;
use types::GlobContext;

fn main() {
    leptos::mount_to_body(App);
}

// users_content.rs: упростить код
#[component]
pub fn App() -> impl IntoView {
    let (session, set_session) = create_signal::<Option<Session>>(None);

    view! {
        <Router>
            {provide_context(GlobContext {
                session,                           // Authorization, Registration, struct UserInfo
                user_resource: create_user_resource(session), // Header, Users
                roles: create_action(move |_: &()| async move { server::fetch_all_roles().await }),
            });}
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header set_session={set_session}/> // btn. "выход" сбрасывает authed_user на None
                <main class="pt-[120px]">
                    <Routes>
                        <Route path="/" view=move || { view!{<div>"Главная страница"</div>} }/>
                        <Route path="/login" view=move || { view!{<Authorization set_session={set_session}/>} }/>
                        <Route path="/register" view=move || { view!{<Registration set_session={set_session}/>} }/>
                        <Route path="/users" view=move || { Users }/>
                        <Route path="/post" view=move || { view!{<div>"Статьи"</div>} }/>
                        <Route path="/post/:postId" view=move || { view!{<div>"Статья"</div>} }/>
                        <Route path="/*" view=move || { view!{<div>"Ошибка"</div>} }/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

fn create_user_resource(
    session: ReadSignal<Option<Session>>,
) -> Resource<(), Option<User>> {
    let current_path = use_location().pathname;
    let (location, set_location) = create_signal::<String>(current_path.get_untracked());

    let user_resource = create_local_resource(
        move || {
            // если нет сессии, обновление локации не происходит
            // if session.with(Option::is_some) {
            //     set_location.set(current_path.get());
            // };
            // (session, location)
            ()
        },
        move |_| {
            session.track();
            location.track();
            if session.with(Option::is_some) {
                set_location.set(current_path.get());
            };

            // logging::log!("user_info.rs: перед запуском async данные пользователя");
            async move {
                match session.get_untracked() {
                    Some(ref sess) => {
                        logging::log!("user_info.rs: async данные пользователя");
                        bff_server::fetch_user(&sess, &sess.user_id).await.unwrap()
                    }
                    None => {
                        logging::log!(
                            "user_info.rs: отклонено обновление пользователя (нет сессии)"
                        );
                        None
                    }
                }
            }
        },
    );

    user_resource
}
