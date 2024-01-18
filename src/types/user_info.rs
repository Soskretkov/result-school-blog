use bff::server::{self as bff_server, Session, User};
use leptos::*;

#[derive(Debug, Clone)]
pub struct UserInfo {
    resurce: Resource<(Option<Session>, String), Option<User>>,
}

impl UserInfo {
    pub fn new(session: ReadSignal<Option<Session>>, location: ReadSignal<String>) -> Self {
        let (changed_location, set_changed_location) = create_signal::<String>(location.get_untracked());

        let user_info = create_local_resource(
            move || {
                // если нет сессии, обновление локации не происходит
                if session.with(Option::is_some) {
                    set_changed_location.set(location.get());
                };
                (session.get(), changed_location.get())
            },
            move |(wrpd_session, _)| async move {
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
