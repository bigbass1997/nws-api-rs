

use serde::{Serialize, Deserialize};
use url::Url;
use wkt::Wkt;
use super::{QuantitativeValue, serialize_wkt, deserialize_wkt};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    #[serde(deserialize_with = "deserialize_wkt", serialize_with = "serialize_wkt")]
    pub geometry: Wkt<f64>,
    #[serde(rename = "@id")]
    pub id: Url,
    #[serde(rename = "@type")]
    pub kind: String,
    pub elevation: QuantitativeValue,
    pub station: Url,
    pub timestamp: String,
    pub raw_message: String,
    pub text_description: String,
    pub icon: Option<Url>, // API marks this as deprecated
    pub present_weather: Vec<MetarPhenomenon>,
    pub temperature: QuantitativeValue,
    pub dewpoint: QuantitativeValue,
    pub wind_direction: QuantitativeValue,
    pub wind_speed: QuantitativeValue,
    pub wind_gust: QuantitativeValue,
    pub barometric_pressure: QuantitativeValue,
    pub sea_level_pressure: QuantitativeValue,
    pub visibility: QuantitativeValue,
    pub max_temperature_last_24_hours: QuantitativeValue,
    pub min_temperature_last_24_hours: QuantitativeValue,
    pub precipitation_last_hour: QuantitativeValue,
    pub precipitation_last_3_hours: QuantitativeValue,
    pub precipitation_last_6_hours: QuantitativeValue,
    pub relative_humidity: QuantitativeValue,
    pub wind_chill: QuantitativeValue,
    pub heat_index: QuantitativeValue,
    pub cloud_layers: Vec<ObservationCloudLayer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObservationCollection {
    #[serde(rename = "@graph")]
    pub graph: Vec<Observation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetarPhenomenon {
    pub intensity: Option<String>,
    pub modifier: Option<String>,
    pub weather: String,
    pub raw_string: String,
    pub in_vicinity: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObservationCloudLayer {
    pub base: QuantitativeValue,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObservationStation {
    #[serde(deserialize_with = "deserialize_wkt", serialize_with = "serialize_wkt")]
    pub geometry: Wkt<f64>,
    #[serde(rename = "@id")]
    pub id: Url,
    #[serde(rename = "@type")]
    pub kind: String,
    pub elevation: QuantitativeValue,
    pub station_identifier: String,
    pub name: String,
    pub time_zone: String,
    pub forecast: Url,
    pub county: Url,
    pub fire_weather_zone: Url,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObservationStationCollection {
    #[serde(rename = "@graph")]
    pub graph: Vec<ObservationStation>,
    pub observation_stations: Vec<Url>,
}

