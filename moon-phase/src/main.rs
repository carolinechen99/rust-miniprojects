// // api_key = "K483SRRFQXY8ZEMKZ4EQXFXBX"

// use reqwest;
// use serde::Deserialize;
// use std::io;

// const BASE_URL: &str = "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline";

// #[derive(Deserialize, Debug)]
// struct ApiResponse {
//     days: Vec<Day>,
// }

// #[derive(Deserialize, Debug)]
// struct Day {
//     moonphase: f64,
// }

// fn moon_phase_name(moonphase: f64) -> (&'static str, f64) {
//     let name = match moonphase {
//         x if (0.0..=0.25).contains(&x) && (x - 0.25).abs() > f64::EPSILON => "waxing crescent",
//         x if (0.25..=0.5).contains(&x) && (x - 0.5).abs() > f64::EPSILON => "waxing gibbous",
//         x if (0.5..=0.75).contains(&x) && (x - 0.75).abs() > f64::EPSILON => "waning gibbous",
//         x if (0.75..=1.0).contains(&x) && (x - 1.0).abs() > f64::EPSILON => "waning crescent",
//         0.0 => "new moon",
//         0.25 => "first quarter",
//         0.5 => "full moon",
//         0.75 => "last quarter",
//         _ => unreachable!(),
//     };
//     (name, moonphase)
// }

// async fn get_moon_phase(api_key: &str, date: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
//     let url = format!(
//         "{base_url}/{date}?unitGroup=us&key={api_key}&include=days&elements=datetime,moonphase,sunrise,sunset&contentType=json",
//         base_url = BASE_URL,
//         date = date,
//         api_key = api_key
//     );

//     let response_text = reqwest::get(&url).await?.text().await?;
//     let response: ApiResponse = serde_json::from_str(&response_text)?;
//     Ok(response)
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let api_key = "K483SRRFQXY8ZEMKZ4EQXFXBX";
//     println!("Enter your birthday (YYYY-MM-DD):");
//     let mut date = String::new();
//     io::stdin().read_line(&mut date).expect("Failed to read input");
//     let date = date.trim();

//     let api_response = get_moon_phase(api_key, date).await?;

//     if let Some(day) = api_response.days.get(0) {
//         let (name, number) = moon_phase_name(day.moonphase);
//         println!("Moon phase on {}: {} ({})", date, name, number);
//     } else {
//         eprintln!("No data found for the specified date.");
//     }

//     Ok(())
// }







use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct ApiResponse {
    days: Vec<Day>,
}

#[derive(Deserialize)]
struct Day {
    // datetime: String,
    moonphase: f64,
}

fn moon_phase_name(moonphase: f64) -> &'static str {
    match moonphase {
        x if (0.0..=0.25).contains(&x) && x != 0.25 => "waxing crescent",
        x if (0.25..=0.5).contains(&x) && x != 0.5 => "waxing gibbous",
        x if (0.5..=0.75).contains(&x) && x != 0.75 => "waning gibbous",
        x if (0.75..=1.0).contains(&x) && x != 1.0 => "waning crescent",
        0.0 => "new moon",
        0.25 => "first quarter",
        0.5 => "full moon",
        0.75 => "last quarter",
        _ => "unknown",
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "K483SRRFQXY8ZEMKZ4EQXFXBX"; // Replace with your Visual Crossing Weather API key
    let base_url = "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/Herndon,VA";

    println!("Enter your birthday (YYYY-MM-DD):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let date = input.trim();

    let url = format!(
        "{base_url}/{date}?unitGroup=us&key={api_key}&include=days&elements=datetime,moonphase,sunrise,sunset&contentType=json",
        base_url = base_url,
        date = date,
        api_key = api_key
    );

    let raw_response = reqwest::get(&url).await?.text().await?;
    // println!("Raw JSON response: {}", raw_response);

    let response: ApiResponse = serde_json::from_str(&raw_response)?;

    if let Some(day) = response.days.first() {
        let moon_phase = moon_phase_name(day.moonphase);
        println!("The moon phase on your birthday was: {moon_phase}");
    } else {
        println!("No moon phase data found for the specified date.");
    }

    Ok(())
}
