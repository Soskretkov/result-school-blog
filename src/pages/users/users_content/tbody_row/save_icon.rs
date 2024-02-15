use crate::components::Icon;
use leptos::*;

#[component]
pub fn SaveIcon(is_selected: bool, #[prop(optional)] class: &'static str) -> impl IntoView {
    let color_class = if is_selected {
        ""
    } else {
        "text-[#CCC]"
    };

    let preset_classes = format!("cursor-pointer text-[24px] mx-2.5 {}", color_class);

    let class_list = format!("{preset_classes} {class}");

    view! {
        <Icon
            id="fa-floppy-o"
            class=class_list
        />
    }
}
