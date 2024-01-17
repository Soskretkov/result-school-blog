use super::H2;
use leptos::*;

#[component]
pub fn PageErrMsg(err_msg: String) -> impl IntoView {
    view! {
        <H2>"Ошибка"</H2>
        <div class="">{err_msg}</div>
    }
}
