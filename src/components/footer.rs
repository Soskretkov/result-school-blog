mod weather;
use leptos::*;
use weather::Weather;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="w-[1000px] h-[120px] flex flex-col justify-between font-bold">
            <div class="h-2 shadow-md"></div>
            <div class="py-5 px-[40px] flex justify-between items-center bg-white shadow-md">
                <div>
                    <div>Блог веб-разработчика</div>
                    <div>"web@developer.ru"</div>
                </div>
                <Weather/>
            </div>
        </div>
    }
}