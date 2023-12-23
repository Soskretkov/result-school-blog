use leptos::*;
use crate::types::session::Session;

#[derive(Debug, Clone)]
pub struct ClobContext {
    pub session: ReadSignal<Option<Session>>,
    pub set_session: WriteSignal<Option<Session>>,
}