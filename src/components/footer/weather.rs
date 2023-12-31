use crate::types::GlobContext;
use chrono::{Datelike, Local};
use gloo_timers::future::TimeoutFuture;
use leptos::*;
use serde::Deserialize;
use serde_json;

#[component]
pub fn Weather() -> impl IntoView {
    let weather_resurce = create_local_resource(
        || use_context::<GlobContext>().unwrap().location.get(),
        |_| {
            logging::log!("Footer (weather.rs): async погода");
            get_weather()
        },
    );

    let formatted_date = || {
        let now = Local::now();
        format!("{} {}", now.day(), month_to_russian(now.month()))
    };

    let sity = move || weather_resurce.with(|data| data.as_ref().map(|data| data.sity.clone()));
    let temperature =
        move || weather_resurce.with(|data| data.as_ref().map(|data| data.temperature.clone()));
    let weather_desc =
        move || weather_resurce.with(|data| data.as_ref().map(|data| data.weather_desc.clone()));

    view! {
        <div>
            // <Transition
            <Suspense
                fallback=|| "загрузка..."
            >
                <div>{sity}", "{formatted_date}</div>
                <div>{temperature}" градусов, "{weather_desc}</div>
            // </Transition>
            </Suspense>
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

#[derive(Deserialize)]
struct RemoteMain {
    temp: f32,
}

#[derive(Deserialize)]
struct RemoteWeather {
    main: RemoteMain,
    name: String,
    weather: Vec<String>,
}

#[derive(Clone, Debug)]
struct Weather {
    sity: String,
    temperature: f32,
    weather_desc: String,
}

async fn get_weather() -> Weather {
    TimeoutFuture::new(1_000).await;

    let json_file_content: &str = include_str!("./weather.json");
    let mut weather: RemoteWeather = serde_json::from_str(json_file_content).unwrap();

    Weather {
        sity: weather.name,
        temperature: weather.main.temp,
        weather_desc: weather.weather.remove(0),
    }
}
