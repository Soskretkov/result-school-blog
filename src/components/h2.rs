use leptos::*;

#[component]
pub fn H2(children: Children) -> impl IntoView {
    view! {
        <h2 class="font-bold text-2xl my-10">{children().nodes}</h2>
    }
}
