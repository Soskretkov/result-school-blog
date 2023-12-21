use leptos::*;

#[component]
pub fn TableRow(children: Children) -> impl IntoView {
    view! {
        <div class="flex items-center">
            {children()}
        </div>
    }
}
