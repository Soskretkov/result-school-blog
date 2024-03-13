use crate::server;
use crate::server::{Role, Session, User, Error};
use bff::server::{self as bff_server};
use leptos::*;
// use leptos_router::*;

#[derive(Clone)]
pub struct GlobContext {
    pub session: Signal<Option<Session>>,
    pub set_session: WriteSignal<Option<Session>>,
    pub user_resource: Resource<(), Option<User>>,
    pub roles: Action<(), Result<Vec<Role>, String>>,
}

impl GlobContext {
    pub fn new(session: Signal<Option<Session>>, set_session: WriteSignal<Option<Session>>) -> Self {
        let roles_action = create_action(move |_: &()| async move { server::fetch_all_roles().await });
        
        Self {
            session, // Header, Authorization, Registration, PageGuard, server.rs
            set_session, // server.rs
            user_resource: Self::create_user_resource(session), // Header, Users, PageGuard, Header
            roles: roles_action, // Users, PageGuard
        }
    }

    fn create_user_resource(session: Signal<Option<Session>>) -> Resource<(), Option<User>> {
        // let current_path = use_location().pathname;
        // let (location, set_location) = create_signal::<String>(current_path.get_untracked());

        let user_resource = create_local_resource(
            move || (),
            move |_| {
                session.track();
                async move {
                    match session.get_untracked() {
                        Some(ref sess) => {
                            logging::log!(
                                "glob_ctx.rs: async скачивание данных пользователя id={}",
                                sess.user_id
                            );
                            bff_server::fetch_user(&sess, &sess.user_id)
                            .await
                            .map_err(|e| {
                                if let Error::InvalidSession = e {
                                    use_context::<GlobContext>().unwrap().set_session.set(None);
                                }
                            })
                            .ok()
                        }
                        None => {
                            logging::log!(
                                "glob_ctx.rs: отклонено обновление пользователя (нет сессии)"
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
