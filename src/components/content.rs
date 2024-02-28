use super::H2;
use leptos::*;

#[component]
pub fn Content(
    children: Children,
    header: &'static str,
    /// в формате тайлвинд
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let preset_classes = "flex items-center flex-col";

    let class_list = format!("{preset_classes} {class}");

    view! {
        <div class=class_list>
            <H2>{header}</H2>
            {children()}
        </div>
    }
}
