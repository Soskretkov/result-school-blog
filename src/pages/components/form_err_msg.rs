use leptos::*;

#[component]
pub fn FormErrMsg(err_signal: ReadSignal<Option<String>>) -> impl IntoView {
    move || match err_signal.get() {
        Some(e) => view! {
            <div class="bg-red-300 text-[18px] mt-2.5 px-2.5 py-2.5">{e}</div>
        }
        .into_view(),
        None => {}.into_view(),
    }
}
