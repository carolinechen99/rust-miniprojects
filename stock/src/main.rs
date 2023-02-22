// use chrono::prelude::*;
// use chrono::{TimeZone, Utc};
// use std::time::{Duration, UNIX_EPOCH};
// use yahoo_finance_api as yahoo;
// use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use actix_web::{get, App, HttpResponse, HttpServer, Responder}; // DELETE

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the stock server!")
}

// // create a function that returns result of /search/{keyword}
// #[get("/search/{keyword}")]
// // use synchronous function to get the response
// async fn search(info: web::Path<String>) -> impl Responder {
//     let source = yahoo::YahooConnector::new();
//     let response = source.search_ticker(&info).await.unwrap();
//     let mut _ticker_found = false;
//     let mut result = String::new();
//     for i in response.quotes {
//         result.push_str(&format!("{}: {}", i.short_name, i.symbol));
//     }
//     HttpResponse::Ok().body(result)
// }

// // create a function that returns result of /latest/{ticker}/{duration}
// #[get("/latest/{ticker}/{duration}")]
// async fn latest(info: web::Path<(String, String)>) -> impl Responder {
//     let source = yahoo::YahooConnector::new();
//     let response = source.get_latest_quotes(&info.0, &info.1).await.unwrap();
//     let result = response.last_quote().unwrap();
//     let time: DateTime<Utc> = UNIX_EPOCH
//         .checked_add(Duration::from_secs(result.timestamp))
//         .unwrap()
//         .into();
//     HttpResponse::Ok().body(format!(
//         "The latest price of {} is ${} at {}",
//         info.0, result.close, time
//     ))
// }

// // create a function that returns result of /history/{ticker}/{start_time}/{end_time}
// #[get("/history/{ticker}/{start_time}/{end_time}")]
// async fn history(info: web::Path<(String, String, String)>) -> impl Responder {
//     let source = yahoo::YahooConnector::new();
//     let start = Utc.with_ymd_and_hms(
//         info.1[0..4].parse().unwrap(),
//         info.1[4..6].parse().unwrap(),
//         info.1[6..8].parse().unwrap(),
//         0,
//         0,
//         0,
//     );
//     let end = Utc.with_ymd_and_hms(
//         info.2[0..4].parse().unwrap(),
//         info.2[4..6].parse().unwrap(),
//         info.2[6..8].parse().unwrap(),
//         23,
//         59,
//         59,
//     );
//     let response = source
//         .get_quote_history(&info.0, start.unwrap(), end.unwrap())
//         .await
//         .unwrap();
//     let result = response.quotes().unwrap();
//     HttpResponse::Ok().body(format!("{result:?}"))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // print the server is running
    println!("Server running on port 8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
            // .service(search)
            // .service(latest)
            // .service(history)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
