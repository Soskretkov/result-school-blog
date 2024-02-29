use leptos::*;

#[component]
pub fn Icon<T: Into<String> + std::default::Default + std::fmt::Display>(
    /// font-awesome id
    id: &'static str,
    /// в формате тайлвинд
    #[prop(optional)] class: T
) -> impl IntoView {
    let preset_classes = "";
    let class_list = format!("{preset_classes} {class}");

    view! {
        <div class=class_list>
            <i class=format!("fa {id}") aria-hidden="true"></i>
        </div>
    }
}
