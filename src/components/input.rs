use leptos::{*, html::Input};

#[component]
pub fn Input(
    #[prop(optional)] r#type: &'static str,
    #[prop(optional)] placeholder: &'static str,
    /// в формате тайлвинд
    #[prop(default = "w-full")] width: &'static str,
    #[prop(optional)] node_ref:  NodeRef<Input>

) -> impl IntoView {
    let preset_classes = "h-[40px] mx-0 mt-0 mb-2.5 py-2.5 px-2.5 border border-solid border-black text-[18px] w-[260px]";

    let class_list = format!("{preset_classes} {width}");

    view! {
        <input
            type=r#type
            placeholder=placeholder
            node_ref=node_ref
            class=class_list
        />        
    }
}
