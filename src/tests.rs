use crate::client::APP_USER_AGENT;
use net::reqwest::Client;

#[cfg(feature = "ipapi")]
#[ctx::test]
async fn ipapi_test() {

  use crate::ipapi;

  assert!(ipapi::my_ip().await.is_ok());
  assert!(ipapi::my_ip_verbose().await.is_ok());
}

#[cfg(feature = "nws")]
#[ctx::test]
async fn nwsapi_test() {

  use crate::nws;
  use obj::Point;

  let pnt = Point::new(41.320361, -72.063304);
  let client = Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()
    .unwrap();
  let res = nws::get_point(&pnt, &client).await.unwrap();
  nws::get_forecast(&res, &client).await.unwrap();
  nws::get_forecast_hourly(&res, &client).await.unwrap();
}

#[cfg(feature = "openai")]
#[ctx::test]
fn openai_test() {
  use crate::openai;
}
