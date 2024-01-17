use super::H2;
use leptos::*;

#[component]
pub fn PageErrMsg(err: String) -> impl IntoView {
    view! {
        <H2>"Ошибка"</H2>
        <div class="">{err}</div>
    }
}
