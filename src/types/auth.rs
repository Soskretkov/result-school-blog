use bff::server::{self as bff_server, Session, User};
use leptos::*;
use leptos_router::*;

#[derive(Debug, Clone)]
pub struct Auth {
    pub session: Session,
    pub user_resource: Resource<String, User>,
}

impl Auth {
    pub fn new(session: Session) -> Self {
        let session_clone = session.clone();
        let user_resource = create_local_resource(
            move || use_location().pathname.get(),
            move |_| {
                // logging::log!("user_info.rs: перед запуском async данные пользователя");
                let session_clone = session_clone.clone();
                async move {
                    logging::log!("user_info.rs: async данные пользователя");
                    bff_server::fetch_user(&session_clone, &session_clone.user_id)
                        .await
                        .expect("auth.rs: паника1")
                        .expect("auth.rs: паника2")
                }
            },
        );

        Self {
            session,
            user_resource,
        }
    }
}