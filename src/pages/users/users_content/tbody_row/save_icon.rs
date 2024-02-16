use crate::components::Icon;
use leptos::*;

#[component]
pub fn SaveIcon(is_deactive: bool, #[prop(optional)] class: &'static str) -> impl IntoView {
    let deactiv_class = if is_deactive { "text-[#CCC]" } else { "" };

    let preset_classes = format!("cursor-pointer text-[24px] mx-2.5 {}", deactiv_class);

    let class_list = format!("{preset_classes} {class}");

    view! {
        <Icon
            id="fa-floppy-o"
            class=class_list
        />
    }
}
