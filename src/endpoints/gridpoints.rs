

use serde::{Serialize, Deserialize};
use url::Url;
use wkt::Wkt;
use crate::endpoints::stations::ObservationStation;
use super::{QuantitativeValue, serialize_wkt, deserialize_wkt, deserialize_force_usize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weather {
    pub values: Vec<WeatherValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WeatherValue {
    pub valid_time: String,
    pub value: Vec<WeatherValueInner>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherValueInner {
    pub coverage: Option<String>,
    pub weather: Option<String>,
    pub intensity: Option<String>,
    pub visibility: QuantitativeValue,
    pub attributes: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hazards {
    pub values: Vec<HazardsValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HazardsValue {
    pub valid_time: String,
    pub value: Vec<HazardsValueInner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HazardsValueInner {
    pub phenomenon: String,
    pub significance: String,
    pub event_number: Option<f64>, // Unlike other structs this should *not* be renamed/aliased using camelCase
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuantitativeValueLayer {
    pub uom: Option<String>,
    pub values: Vec<QuantitativeValueLayerValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuantitativeValueLayerValue {
    pub valid_time: String,
    pub value: Option<f64>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gridpoint {
    #[serde(deserialize_with = "deserialize_wkt", serialize_with = "serialize_wkt")]
    pub geometry: Wkt<f64>,
    #[serde(rename = "@id")]
    pub id: Url,
    #[serde(rename = "@type")]
    pub kind: String,
    pub update_time: String,
    pub valid_times: String,
    pub elevation: QuantitativeValue,
    pub forecast_office: Url,
    pub grid_id: String,
    #[serde(deserialize_with = "deserialize_force_usize")]
    pub grid_x: usize,
    #[serde(deserialize_with = "deserialize_force_usize")]
    pub grid_y: usize,
    pub weather: Option<Weather>,
    pub hazards: Option<Hazards>,
    
    pub temperature: Option<QuantitativeValueLayer>,
    pub dewpoint: Option<QuantitativeValueLayer>,
    pub max_temperature: Option<QuantitativeValueLayer>,
    pub min_temperature: Option<QuantitativeValueLayer>,
    pub relative_humidity: Option<QuantitativeValueLayer>,
    pub apparent_temperature: Option<QuantitativeValueLayer>,
    pub heat_index: Option<QuantitativeValueLayer>,
    pub wind_chill: Option<QuantitativeValueLayer>,
    pub sky_cover: Option<QuantitativeValueLayer>,
    pub wind_direction: Option<QuantitativeValueLayer>,
    pub wind_speed: Option<QuantitativeValueLayer>,
    pub wind_gust: Option<QuantitativeValueLayer>,
    pub probability_of_precipitation: Option<QuantitativeValueLayer>,
    pub quantitative_precipitation: Option<QuantitativeValueLayer>,
    pub ice_accumulation: Option<QuantitativeValueLayer>,
    pub snowfall_amount: Option<QuantitativeValueLayer>,
    pub snow_level: Option<QuantitativeValueLayer>,
    pub ceiling_height: Option<QuantitativeValueLayer>,
    pub visibility: Option<QuantitativeValueLayer>,
    pub transport_wind_speed: Option<QuantitativeValueLayer>,
    pub transport_wind_direction: Option<QuantitativeValueLayer>,
    pub mixing_height: Option<QuantitativeValueLayer>,
    pub haines_index: Option<QuantitativeValueLayer>,
    pub lightning_activity_level: Option<QuantitativeValueLayer>,
    pub twenty_foot_wind_speed: Option<QuantitativeValueLayer>,
    pub twenty_foot_wind_direction: Option<QuantitativeValueLayer>,
    pub wave_height: Option<QuantitativeValueLayer>,
    pub wave_period: Option<QuantitativeValueLayer>,
    pub wave_direction: Option<QuantitativeValueLayer>,
    pub primary_swell_height: Option<QuantitativeValueLayer>,
    pub primary_swell_direction: Option<QuantitativeValueLayer>,
    pub secondary_swell_height: Option<QuantitativeValueLayer>,
    pub secondary_swell_direction: Option<QuantitativeValueLayer>,
    pub wave_period2: Option<QuantitativeValueLayer>,
    pub wind_wave_height: Option<QuantitativeValueLayer>,
    pub dispersion_index: Option<QuantitativeValueLayer>,
    pub pressure: Option<QuantitativeValueLayer>,
    pub probability_of_trapical_storm_winds: Option<QuantitativeValueLayer>,
    pub probability_of_hurricane_winds: Option<QuantitativeValueLayer>,
    pub potential_of_15mph_winds: Option<QuantitativeValueLayer>,
    pub potential_of_25mph_winds: Option<QuantitativeValueLayer>,
    pub potential_of_35mph_winds: Option<QuantitativeValueLayer>,
    pub potential_of_45mph_winds: Option<QuantitativeValueLayer>,
    pub potential_of_20mph_wind_gusts: Option<QuantitativeValueLayer>,
    pub potential_of_30mph_wind_gusts: Option<QuantitativeValueLayer>,
    pub potential_of_40mph_wind_gusts: Option<QuantitativeValueLayer>,
    pub potential_of_50mph_wind_gusts: Option<QuantitativeValueLayer>,
    pub potential_of_60mph_wind_gusts: Option<QuantitativeValueLayer>,
    pub grassland_fire_danger_index: Option<QuantitativeValueLayer>,
    pub probability_of_thunder: Option<QuantitativeValueLayer>,
    pub davis_stability_index: Option<QuantitativeValueLayer>,
    pub atmospheric_dispersion_index: Option<QuantitativeValueLayer>,
    pub stability: Option<QuantitativeValueLayer>,
    pub red_flag_thread_index: Option<QuantitativeValueLayer>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GridpointStations {
    #[serde(rename = "@graph")]
    pub graph: Vec<ObservationStation>,
    pub observation_stations: Vec<Url>,
}








