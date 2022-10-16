use std::env;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_wasm() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn log_weather_data() {
    match get_weather_data().await {
        Ok(data) => log(&data),
        Err(error) => panic!("{}", error),
    }
}

fn get_query_url(location: &str) -> String {
    let api_key = env!("WEATHER_API_KEY");
    let api_url =
        "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline";
    format!("{api_url}/{location}?unitGroup=metric&include=hours&key={api_key}&contentType=json")
}

async fn get_weather_data() -> Result<String, reqwest::Error> {
    reqwest::get(get_query_url("Dublin")).await?.text().await
}
