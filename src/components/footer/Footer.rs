use leptos::*;
use serde_json;


struct Weather {
    main: &'static str,
    name: &'static str,
    weather: &'static str,
}

#[component]
pub fn Footer() -> impl IntoView {
    let file: &str = include_str!("./weather.json");
    // let weather: Weather = serde_json::from_str(file).unwrap();


    // let weather = Weather {
    //     main: "",
    //     name: "",
    //     weather: "",
    // };



    let (sity, serSity) = leptos::create_signal("");
    let (temperature, serTemperature) = leptos::create_signal("");
    let (weather, serWeather) = leptos::create_signal("");

    view! {
        <h1>Footer</h1>
    }
}