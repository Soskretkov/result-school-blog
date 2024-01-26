use bff::server::{self as bff_server, Session, User};
use leptos::*;
use leptos_router::*;

#[derive(Debug, Clone, Copy)]
pub struct UserInfo {
    resurce: Resource<(Option<Session>, String), Option<User>>,
}

impl UserInfo {
    pub fn new(session: ReadSignal<Option<Session>>) -> Self {
        let current_path = use_location().pathname;
        let (location, set_location) =
            create_signal::<String>(current_path.get_untracked());

        let user_info = create_local_resource(
            move || {
                // если нет сессии, обновление локации не происходит
                if session.with(Option::is_some) {
                    set_location.set(current_path.get());
                };
                (session.get(), location.get())
            },
            move |(wrpd_session, _)| {
                // logging::log!("user_info.rs: перед запуском async данные пользователя");
                async move {
                    match wrpd_session {
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

        Self { resurce: user_info }
    }

    pub fn track(&self) {
        self.resurce.track()
    }

    pub fn is_loaded(&self) -> bool {
        self.resurce
            .with(move |ui| ui.as_ref().map(Option::is_some).unwrap_or(false))
    }

    pub fn user_data(&self) -> Option<User> {
        self.resurce.get().and_then(|user_info| user_info)
    }

    // pub fn user_data(&self) -> Option<&UserData> {
    //     self.resurce.with(|rf| rf.as_ref().and_then(Option::as_ref))
    // }

    // pub fn refetch(&mut self) {
    //     self.resurce.refetch();
    // }
}
