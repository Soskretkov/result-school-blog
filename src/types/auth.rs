use bff::server::{self as bff_server, Session, User};
use leptos::*;

#[derive(Debug, Clone)]
pub struct Auth {
    pub session: Session,
    pub user_resource: UserResource,
}

impl Auth {
    pub fn new(session: Session) -> Self {
        Self {
            session,
            user_resource: UserResource::new(&session),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct UserResource {
    resurce: Resource<(), Option<User>>,
}

impl UserResource {
    pub fn new(session: &Session) -> Self {
        let user_info = create_local_resource(
            move || (),
            move |_| {
                // logging::log!("user_info.rs: перед запуском async данные пользователя");
                async move {
                    logging::log!("user_info.rs: async данные пользователя");
                    bff_server::fetch_user(session, &session.user_id)
                        .await
                        .unwrap()
                }
            },
        );

        Self { resurce: user_info }
    }

    pub fn track(&self) {
        self.resurce.track()
    }

    pub fn is_loaded(&self) -> bool {
        self.resurce
            .with(move |ui| ui.as_ref().map(Option::is_some).unwrap_or(false))
    }

    pub fn get_data(&self) -> Option<User> {
        self.resurce.get().and_then(|user_info| user_info)
    }

    // pub fn user_data(&self) -> Option<&UserData> {
    //     self.resurce.with(|rf| rf.as_ref().and_then(Option::as_ref))
    // }

    // pub fn refetch(&mut self) {
    //     self.resurce.refetch();
    // }
}
