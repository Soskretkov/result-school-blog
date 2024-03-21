use crate::types::GlobContext;
use leptos::*;

#[derive(Clone)]
pub struct ModalWindow {
    pub text: String,
    pub on_confirm: Callback<()>,
    pub on_cancel: Callback<()>,
}

impl ModalWindow {
    fn new<T>(text: String, on_confirm: T) -> Self
    where
        T: Fn() + 'static,
    {
        let glob_ctx = use_context::<GlobContext>().unwrap();

        Self {
            text,
            on_confirm: Callback::from(move |_: ()| {
                // закрыть модальное окно
                glob_ctx.set_modal_window.set(None);
                on_confirm();
            }),
            on_cancel: Callback::from(move |_: ()| {
                // закрыть модальное окно
                glob_ctx.set_modal_window.set(None);
            }),
        }
    }
    pub fn set<T>(text: String, on_confirm: T)
    where
        T: Fn() + 'static,
    {
        let glob_ctx = use_context::<GlobContext>().unwrap();
        let modal = ModalWindow::new(text, on_confirm);

        glob_ctx.set_modal_window.set(Some(modal));
    }
}
