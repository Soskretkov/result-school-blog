use super::Button;
use leptos::*;

#[component]
pub fn Modal<F>(text: String, onConfirm: F, onCancel: F) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    let preset_classes = "";
    let class_list = format!("{preset_classes}");

    view! {
        <div> // Modal class
            <div></div> // owerlay class
            <div> // box class
                <h3>"удалить комментарий?"{text}</h3>
                <div> // buttons class
                    <Button on:click=onConfirm class="w-[120px]">"Да"</Button>
                    <Button on:click=onCancel class="w-[120px]">"Отмена"</Button>
                </div>
            </div>
        </div>
    }
}
