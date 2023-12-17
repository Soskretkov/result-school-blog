use leptos::*;

#[component]
pub fn FormErrMsg() -> impl IntoView {
    view! {
        <div class="bg-red-300 text-[18px] mt-2.5 px-2.5 py-2.5">"Заполните логин"</div>
    }
}