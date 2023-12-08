use leptos::*;
use crate::components::Icon;

#[component]
pub fn ControlPanel() -> impl IntoView {
    view! {
        <div>            
            <div class="flex justify-end">
                <button class="text-[18px] w-[100px] h-[32px]" >Войти</button>
            </div>
            <div class="flex justify-end">
                <Icon id="fa-backward" class="text-[24px] mt-[10px]"/>
                <Icon id="fa-file-text-o" class="text-[24px] mt-[10px] ml-[17px]"/>
                <Icon id="fa-users" class="text-[24px] mt-[10px] ml-[17px]"/>
            </div>            
        </div>
    }
}