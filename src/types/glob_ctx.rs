use super::{Auth, ModalConfig};
use crate::server;
use crate::server::Role;
use leptos::*;

#[derive(Clone)]
pub struct GlobContext {
    pub auth: Signal<Option<Auth>>,
    pub set_auth: WriteSignal<Option<Auth>>,
    pub roles: Action<(), Result<Vec<Role>, String>>,
    pub set_modal_cfg: WriteSignal<Option<ModalConfig>>,
}

impl GlobContext {
    pub fn new(
        auth: Signal<Option<Auth>>,
        set_auth: WriteSignal<Option<Auth>>,
        set_modal_cfg: WriteSignal<Option<ModalConfig>>,
    ) -> Self {
        let roles_action =
            create_action(move |_: &()| async move { server::fetch_all_roles().await });

        Self {
            auth,                // Header, Authorization, Registration, PageGuard, server.rs
            set_auth,            // server.rs
            roles: roles_action, // Users, PageGuard
            set_modal_cfg,
        }
    }
}
