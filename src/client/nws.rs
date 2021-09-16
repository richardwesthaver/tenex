use super::APP_USER_AGENT;
use net::reqwest::Client;
use crate::Error;
use chrono::{DateTime, Utc};
use logger::log::debug;
use obj::Point;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// City object
///
/// Used to parse City metadata from datasets acquired on the internet
#[derive(Deserialize, Serialize, Debug)]
pub struct City {
  pub city: String,
  pub state_id: String,
  pub lat: f32,
  pub lng: f32,
}

impl City {
  /// Convert a City to Point.
  ///
  /// Returns Ok(Point) on success. Note that only f32 values are
  /// accepted (0. 1. -- not 0 1).
  pub fn into_point(&self) -> Result<Point, obj::Error> {
    Ok(Point {
      lat: self.lat,
      lng: self.lng,
    })
  }
}

/// Result of a GET /point request
#[derive(Serialize, Deserialize, Debug)]
pub struct PointInfo {
  id: String,
  pub properties: PointProps,
}

/// Inner properties object of PointInfo
#[derive(Serialize, Deserialize, Debug)]
pub struct PointProps {
  #[serde(rename(deserialize = "forecastOffice"))]
  pub forecast_office: String,
  pub forecast: String,
  #[serde(rename(deserialize = "forecastHourly"))]
  pub forecast_hourly: String,
  #[serde(rename(deserialize = "forecastGridData"))]
  pub forecast_grid_data: String,
  #[serde(rename(deserialize = "observationStations"))]
  pub observation_stations: String,
  #[serde(rename(deserialize = "relativeLocation"))]
  pub relative_location: RelativeLocation,
  #[serde(rename(deserialize = "forecastZone"))]
  pub forecast_zone: String,
  pub county: String,
  #[serde(rename(deserialize = "fireWeatherZone"))]
  pub fire_weather_zone: String,
  #[serde(rename(deserialize = "timeZone"))]
  pub time_zone: String,
  #[serde(rename(deserialize = "radarStation"))]
  pub radar_station: String,
}

/// inner relative_location object of PointProps
#[derive(Debug, Serialize, Deserialize)]
pub struct RelativeLocation {
  pub geometry: Value,
  pub properties: RelativeProps,
}

/// inner properties object of RelativeLocation
#[derive(Debug, Serialize, Deserialize)]
pub struct RelativeProps {
  pub city: String,
  pub state: String,
  pub distance: Value,
  pub bearing: Value,
}

/// Result of GET /forecast
#[derive(Debug, Serialize, Deserialize)]
pub struct Forecast {
  pub properties: ForecastProps,
}

/// Inner properties object of Forecast
#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastProps {
  pub updated: DateTime<Utc>,
  pub units: String,
  #[serde(rename(deserialize = "generatedAt"))]
  pub generated_at: DateTime<Utc>,
  pub elevation: Value,
  pub periods: Vec<ForecastPeriod>,
}

/// Single instance of item in periods object of ForecastProps
#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastPeriod {
  pub number: u16,
  pub name: String,
  #[serde(rename(deserialize = "startTime"))]
  pub start_time: DateTime<Utc>,
  #[serde(rename(deserialize = "endTime"))]
  pub end_time: DateTime<Utc>,
  #[serde(rename(deserialize = "isDaytime"))]
  pub is_day_time: bool,
  pub temperature: i8,
  #[serde(rename(deserialize = "temperatureUnit"))]
  pub temperature_unit: String,
  #[serde(rename(deserialize = "windSpeed"))]
  pub wind_speed: Option<String>,
  #[serde(rename(deserialize = "windDirection"))]
  pub wind_direction: Option<String>,
  pub icon: String,
  #[serde(rename(deserialize = "shortForecast"))]
  pub short_forecast: String,
  #[serde(rename(deserialize = "detailedForecast"))]
  pub detailed_forecast: String,
}

/// Forecast output representation
#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastBundle {
  pub start: DateTime<Utc>,
  pub end: DateTime<Utc>,
  pub temperature: i8,
  pub wind_speed: String, // TODO parse from string to int "30 mph" -> 30
  pub wind_direction: String,
  pub short_forecast: String,
}

/// WeatherForecast output representation tied to a specific City.
///
/// This struct is passed directly into an embedded Database
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherBundle {
  pub location: City,
  pub forecast: Vec<ForecastBundle>,
  pub updated: DateTime<Utc>,
}

impl WeatherBundle {
  /// Create a new WeatherBundle from a City and Forecast
  pub fn new(loc: City, fcb: Forecast) -> Self {
    let mut vec = Vec::new();
    for i in fcb.properties.periods.iter() {
      let i = ForecastBundle {
        start: i.start_time,
        end: i.end_time,
        temperature: i.temperature,
        wind_speed: i.wind_speed.as_ref().unwrap().to_string(),
        wind_direction: i.wind_direction.as_ref().unwrap().to_string(),
        short_forecast: i.short_forecast.to_string(),
      };
      vec.push(i);
    }
    WeatherBundle {
      location: loc,
      forecast: vec,
      updated: fcb.properties.updated,
    }
  }
}

pub async fn get_point(pnt: &Point, client: &Client) -> Result<PointInfo, Error> {
  let mut url: String = String::from("http://api.weather.gov/");
  for i in &["points/", &pnt.lat.to_string(), ",", &pnt.lng.to_string()] {
    url.push_str(i);
  }
  let response = client.get(&url).send().await?;
  let body = response.text().await?;
  debug!("{}", body);
  let res: PointInfo = serde_json::from_str(&body)?;
  Ok(res)
}

pub async fn get_forecast(pnt: &PointInfo, client: &Client) -> Result<Forecast, Error> {
  let response = client.get(&pnt.properties.forecast).send().await?;
  let body = response.text().await?;
  debug!("{}", body);
  let res: Forecast = serde_json::from_str(&body)?;
  Ok(res)
}

pub async fn get_forecast_hourly(pnt: &PointInfo, client: &Client) -> Result<Forecast, Error> {
  let response = client.get(&pnt.properties.forecast_hourly).send().await?;
  let body = response.text().await?;
  let res: Forecast = serde_json::from_str(&body)?;
  Ok(res)
}

/// TODO [2021-08-21] - get_alerts
pub async fn get_alerts(_state: &str) -> Result<(), Error> {
  Ok(())
}

pub async fn weather_report(lat: f32, lng: f32) -> Result<(), Error> {
  let client = Client::builder().user_agent(APP_USER_AGENT).build()?;

  let point = Point { lat, lng };

  let res = get_point(&point, &client).await?;
  let resf = get_forecast(&res, &client).await?;

  for i in resf.properties.periods.iter() {
    println!(
      "{:?} -- [{:#?} - {:#?}]\ntemperature: {:#?}\n{:#?}\n",
      &i.name, &i.start_time, &i.end_time, &i.temperature, &i.short_forecast
    );
    println!("------------\n");
  }
  Ok(())
}
