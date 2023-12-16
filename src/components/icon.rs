use leptos::*;

// я так понял у них выравниваение по левому верхнему углу
#[component]
pub fn Icon(id: &'static str, #[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div class=class>
            <i class=format!("fa {id}") aria-hidden="true"></i>
        </div>
    }
}
