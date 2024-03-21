use super::Button;
use crate::types::ModalWindow;
use leptos::*;

#[component]
pub fn Modal(modal_window: ReadSignal<Option<ModalWindow>>) -> impl IntoView {
    view! {
        {move || match modal_window.get() {
            Some(window) => view! {
                <div class="fixed z-20 top-0 right-0 bottom-0 left-0"> // Modal class
                    <div class="absolute w-full h-full bg-[rgba(0,0,0,0.7)]"></div> // overlay class
                    <div class="relative top-[40%] w-[400px] mt-0 mx-auto pt-0 px-5 pb-5 text-center bg-[#fff] border-[3px] border-solid border-black z-30"> // box class
                        <h3 class="font-bold py-[18px]">{window.text}</h3>
                        <div class="flex justify-center"> // buttons class
                            <Button class="w-[120px] mx-[5px]" on:click=move |_| window.on_confirm.call(())>"Да"</Button>
                            <Button class="w-[120px] mx-[5px]" on:click=move |_| window.on_cancel.call(())>"Отмена"</Button>
                        </div>
                    </div>
                </div>
            }.into_view(),
            None => {}.into_view(),
        }}
    }
}
