use super::Button;
use crate::types::ModalConfig;
use leptos::*;

#[component]
pub fn Modal(modal_cfg: ReadSignal<Option<ModalConfig>>) -> impl IntoView {
    view! {
        {move || match modal_cfg.get() {
            Some(modal_config) => view! {
                <div> // Modal class
                    <div></div> // owerlay class
                    <div> // box class
                        <h3>"удалить комментарий?"{modal_config.text}</h3>
                        <div> // buttons class
                            <Button on:click=move |ev| modal_config.on_confirm.call(ev) class="w-[120px]">"Да"</Button>
                            <Button on:click=move |ev| modal_config.on_cancel.call(ev) class="w-[120px]">"Отмена"</Button>
                        </div>
                    </div>
                </div>
            }.into_view(),
            None => {}.into_view(),
        }}
    }
}
