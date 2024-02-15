mod weather;
use leptos::*;
use weather::Weather;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        // высоту сделал меньше чем в оригинальной верстке в видео (плохо смотрелось на моем ноуте)
        // в оригинале h-[120px] и в дочернем элементе py-[20px]
        <div class="w-[1000px] h-[80px] flex flex-col justify-between font-bold">
            <div class="h-2 shadow-md"></div>
            <div class="py-[10px] px-[40px] flex justify-between items-center bg-white shadow-md">
                <div>
                    <div>Блог веб-разработчика</div>
                    <div>"web@developer.ru"</div>
                </div>
                <Weather/>
            </div>
        </div>
    }
}