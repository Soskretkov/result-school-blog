use super::components::PageErrMsg;
use leptos::*;

#[component]
pub fn ErrorPage(err_msg: String) -> impl IntoView {
    view! {
        <PageErrMsg err_msg={err_msg}/>
    }
}
