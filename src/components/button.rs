use leptos::{*, html::Button};

#[component]
pub fn Button<T: Into<String> + std::default::Default + std::fmt::Display>(
    #[prop(optional)] r#type: &'static str,
    /// в формате тайлвинд
    #[prop(optional)] disabled: bool,
    #[prop(optional)] node_ref: NodeRef<Button>,
    #[prop(optional)] class: T,
    children: leptos::Children,


) -> impl IntoView {
    let preset_classes = "flex border border-gray-600 bg-[buttonface] rounded-sm justify-center items-center text-[18px] h-8 cursor-pointer text-black";

    let class_list = format!("{preset_classes} {class}");

    view! {
        <button
            type=r#type
            disabled=disabled
            node_ref=node_ref
            class=class_list
        >
        {children().nodes}
        </button>     
    }
}
