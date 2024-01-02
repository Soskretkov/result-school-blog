mod role;
mod user_data;
use bff::server::Session;
use leptos::*;
pub use role::RoleName;
pub use user_data::UserData;

#[derive(Debug, Clone)]
pub struct UserInfo {
    resurce: Resource<(Option<Session>, String), Option<UserData>>,
}

impl UserInfo {
    pub fn new(session: ReadSignal<Option<Session>>, location: ReadSignal<String>) -> Self {
        let user_info = create_local_resource(
            move || (session.get(), location.get()),
            move |(wrpd_session, _)| async move {            
                logging::log!("user_info.rs: async данные пользователя");
                match wrpd_session {
                    Some(ref sess) => bff::server::fetch_self_user_info(sess)
                        .await
                        .map(|user| UserData::new(user)),
                    None => None,
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
            .with(move |ui| ui.as_ref().map(|f| f.is_some()).unwrap_or(false))
    }

    pub fn user_data(&self) -> Option<UserData> {
        self.resurce.get().and_then(|user_info| user_info)
    }

    // pub fn user_data(&self) -> Option<&UserData> {
    //     self.resurce.with(|rf| rf.as_ref().and_then(Option::as_ref))
    // }

    // pub fn refetch(&mut self) {
    //     self.resurce.refetch();
    // }
}
