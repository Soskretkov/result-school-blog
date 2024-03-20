use leptos::*;

#[derive(Clone)]
pub struct ModalConfig {
    pub text: String,
    pub on_confirm: Callback<()>,
    pub on_cancel: Callback<()>,
}
