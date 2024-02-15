use leptos::*;

#[component]
pub fn Icon(id: &'static str, #[prop(optional)] class: String) -> impl IntoView {
    let preset_classes = "";
    let class_list = format!("{preset_classes} {class}");

    view! {
        <div class=class_list>
            <i class=format!("fa {id}") aria-hidden="true"></i>
        </div>
    }
}
