

use serde::{Serialize, Deserialize};
use url::Url;
use wkt::Wkt;
use super::{QuantitativeValue, serialize_wkt, deserialize_wkt, deserialize_force_usize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelativeLocation {
    pub city: String,
    pub state: String,
    #[serde(deserialize_with = "deserialize_wkt", serialize_with = "serialize_wkt")]
    pub geometry: Wkt<f64>,
    pub distance: QuantitativeValue,
    pub bearing: QuantitativeValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    #[serde(deserialize_with = "deserialize_wkt", serialize_with = "serialize_wkt")]
    pub geometry: Wkt<f64>,
    #[serde(rename = "@id")]
    pub id: Url,
    #[serde(rename = "@type")]
    pub kind: String,
    pub cwa: String,
    pub forecast_office: Url,
    pub grid_id: String,
    #[serde(deserialize_with = "deserialize_force_usize")]
    pub grid_x: usize,
    #[serde(deserialize_with = "deserialize_force_usize")]
    pub grid_y: usize,
    pub forecast: Url,
    pub forecast_hourly: Url,
    pub forecast_grid_data: Url,
    pub observation_stations: Url,
    pub relative_location: RelativeLocation,
    pub forecast_zone: Url,
    pub county: Url,
    pub fire_weather_zone: Url,
    pub time_zone: String,
    pub radar_station: String,
}