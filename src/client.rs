use std::env;

pub use reqwest::{Client, Error, StatusCode};

pub mod ipapi;
pub mod nws;

/// User-Agent HTTP Header value
pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[ctx::test]
async fn ipapi_test() {
  assert!(ipapi::my_ip().await.is_ok());
  assert!(ipapi::my_ip_verbose().await.is_ok());
}

#[ctx::test]
async fn nwsapi_test() {
  let pnt = Point::new(41.320361, -72.063304);
  let client = Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()
    .unwrap();
  let res = nws::get_point(&pnt, &client).await.unwrap();
  nws::get_forecast(&res, &client).await.unwrap();
  nws::get_forecast_hourly(&res, &client).await.unwrap();
}
