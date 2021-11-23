use rlib::net::reqwest::Client;

#[cfg(feature = "ipapi")]
#[tokio::test]
async fn ipapi_test() {
  use crate::ipapi;

  assert!(ipapi::my_ip().await.is_ok());
  assert!(ipapi::my_ip_verbose().await.is_ok());
}

#[cfg(feature = "nws")]
#[tokio::test]
async fn nwsapi_test() {
  use crate::nws;
  use rlib::obj::Point;

  let pnt = Point::new(41.320361, -72.063304);
  let client = Client::builder().build().unwrap();
  let res = nws::get_point(&pnt, &client).await.unwrap();
  nws::get_forecast(&res, &client).await.unwrap();
  nws::get_forecast_hourly(&res, &client).await.unwrap();
}

#[cfg(feature = "openai")]
#[tokio::test]
async fn openai_test() {
  use crate::openai;
}
