use leptos::{*, html::Button};

#[component]
pub fn Button(
    #[prop(optional)] r#type: &'static str,
    /// в формате тайлвинд
    #[prop(default = "w-full")] width: &'static str,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] node_ref: NodeRef<Button>,
    children: leptos::Children,


) -> impl IntoView {
    let class = "flex justify-center items-center text-[18px] h-8 cursor-pointer text-black";

    let class_list = format!("{class} {width}");

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
