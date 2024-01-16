use crate::components::page_guard::Protected;
use leptos::*;
mod users;
use crate::types::GlobContext;
use crate::utils::isSyncServerClientRoles;
use users::Users;

pub struct UsersPage;

impl Protected for UsersPage {
    fn view(&self) -> View {
        view! {
            <Users/>
        }
    }

    fn can_access(&self) -> bool {
        let (can_access, set_access) = create_signal(false);
        let glob_ctx = use_context::<GlobContext>().unwrap().user_info.is_loaded();

        let action = create_action(move |_: &()| async move {
            let is_sync_roles = isSyncServerClientRoles().await;
            set_access.set(is_sync_roles);
        });

        //синхронизация
        //роль
        true
    }
}
