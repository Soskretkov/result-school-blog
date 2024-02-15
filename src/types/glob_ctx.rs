use crate::server;
use crate::server::{Role, Session, User};
use bff::server::{self as bff_server};
use leptos::*;
// use leptos_router::*;

#[derive(Clone)]
pub struct GlobContext {
    pub session: ReadSignal<Option<Session>>,
    pub user_resource: Resource<(), Option<User>>,
    pub roles: Action<(), Result<Vec<Role>, String>>,
}

impl GlobContext {
    pub fn new(session: ReadSignal<Option<Session>>) -> Self {
        Self {
            session, // Authorization, Registration, struct UserInfo
            user_resource: Self::create_user_resource(session), // Header, Users
            roles: create_action(move |_: &()| async move { server::fetch_all_roles().await }),
        }
    }

    fn create_user_resource(session: ReadSignal<Option<Session>>) -> Resource<(), Option<User>> {
        // let current_path = use_location().pathname;
        // let (location, set_location) = create_signal::<String>(current_path.get_untracked());

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
                // location.track();
                // if session.with(Option::is_some) {
                //     set_location.set(current_path.get());
                // };

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
}
