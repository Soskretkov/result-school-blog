mod role;
mod user_data;
use bff::server::{Session};
use leptos::*;
pub use role::RoleName;
pub use user_data::UserData;

#[derive(Debug, Clone)]
pub struct UserInfo {
    resurce: Resource<Option<Session>, Option<UserData>>,
}

impl UserInfo {
    pub fn new(rw_session: RwSignal<Option<Session>>) -> Self {
        let user_info = create_local_resource(
            move || rw_session.get(),
            move |wrpd_session: Option<Session>| async move {
                logging::log!("обновление данных пользователя");
                match wrpd_session {
                    Some(ref session) => bff::server::fetch_self_user_info(session)
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

    pub fn refetch(&mut self) {
        self.resurce.refetch();
    }
}