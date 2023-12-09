use leptos::*;

#[component]
pub fn Icon(id: &'static str, #[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div class=class>
            <i class=format!("fa {}", id) aria-hidden="true"></i>
        </div>
    }
}
