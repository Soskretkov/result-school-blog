use leptos::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div class="flex -mt-3">
            <div class="text-[70px] mr-2.5">
                <i class="fa fa-code" aria-hidden="true"></i>
            </div>
            <div class="">
                <div class="text-[48px] font-semibold leading-[48px] mt-4">"Блог"</div>
                <div class="text-[18px] font-bold">"веб-разработка"</div>
            </div>
        </div>
    }
}
