use leptos::*;
use serde::Deserialize;
use serde_json;
use chrono::{Local, Datelike};

#[derive(Deserialize)]
struct Main {
    temp: f32,
}

#[derive(Deserialize)]
struct Weather {
    main: Main,
    name: String,
    weather: Vec<String>,
}

#[component]
pub fn Footer() -> impl IntoView {
    let json_file_content: &str = include_str!("./footer/weather.json");
    let weather: Weather = serde_json::from_str(json_file_content).unwrap();

    let temperature = weather.main.temp;
    let sity = weather.name;
    let weather_desc = &weather.weather[0];

    let now = Local::now();
    let formatted_date = format!("{} {}", now.day(), month_to_russian(now.month()));

    view! {
        <div class="w-[1000px] h-[120px] flex flex-col justify-between font-bold">
            <div class="h-2 shadow-md"></div>
            <div class="py-5 px-[40px] flex justify-between items-center bg-white shadow-md">
                <div>
                    <div>Блог веб-разработчика</div>
                    <div>"web@developer.ru"</div>
                </div>
                <div>
                    <div>{sity}, {formatted_date}</div>
                    <div>{temperature}" градусов, "{weather_desc}</div>
                </div>
            </div>
        </div>
    }
}


fn month_to_russian(month: u32) -> &'static str {
    match month {
        1 => "января",
        2 => "февраля",
        3 => "марта",
        4 => "апреля",
        5 => "мая",
        6 => "июня",
        7 => "июля",
        8 => "августа",
        9 => "сентября",
        10 => "октября",
        11 => "ноября",
        12 => "декабря",
        _ => "",
    }
}





