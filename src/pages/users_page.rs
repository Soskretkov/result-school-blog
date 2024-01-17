use super::Users;
use crate::components::page_guard::Protected;
use crate::types::GlobContext;
use leptos::*;

pub struct UsersPage;

impl Protected for UsersPage {
    fn view(&self) -> View {
        view! {
            <Users/>
        }
    }

    fn can_access(&self) -> bool {
        // let (can_access, set_access) = create_signal(false);
        match use_context::<GlobContext>().unwrap().user_info.user_data() {
            Some(user) => {
                let role = user.role_id;
                role.can_view_users() && role.can_view_roles()
            }
            None => false,
        }
    }
}
