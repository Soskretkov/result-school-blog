use super::PageErrMsg;
use leptos::*;

#[component]
pub fn Content(children: ChildrenFn, err_signal: ReadSignal<Option<String>>) -> impl IntoView {
    move || match err_signal.get() {
        Some(e) => view! {
            <PageErrMsg err_msg={e}/>
        }
        .into_view(),
        None => {}.into_view(),
    }
}
