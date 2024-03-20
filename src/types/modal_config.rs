use leptos::*;

#[derive(Clone)]
pub struct ModalConfig {
    pub text: String,
    pub on_confirm: Callback<ev::MouseEvent>,
    pub on_cancel: Callback<ev::MouseEvent>,
}
