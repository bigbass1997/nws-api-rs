

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::Value;
use wkt::Wkt;

pub mod gridpoints;
pub mod points;
pub mod stations;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuantitativeValue {
    pub unit_code: Option<String>,
    pub value: Option<f64>,
    pub max_value: Option<f64>,
    pub min_value: Option<f64>,
    pub quality_control: Option<String>
}


fn serialize_wkt<S>(wkt: &Wkt<f64>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let s = wkt.items.first().unwrap().to_string();
    
    s.serialize(serializer)
}

fn deserialize_wkt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Wkt<f64>, D::Error> {
    let buf = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(err) => return Err(err),
    };
    
    match Wkt::from_str(&buf) {
        Ok(wkt) => Ok(wkt),
        Err(_) => return Err(serde::de::Error::custom("Invalid WKT geometry string")),
    }
}

fn deserialize_force_usize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<usize, D::Error> {
    match Value::deserialize(deserializer) {
        Ok(ok) => match ok {
            Value::String(s) => match s.parse() {
                Ok(ok) => Ok(ok),
                Err(_) => Err(serde::de::Error::custom("Failed to parse string into u64")) 
            },
            Value::Number(num) => match num.as_u64() {
                Some(ok) => Ok(ok as usize),
                None => Err(serde::de::Error::custom("Failed to parse number into u64"))
            },
            _ => Err(serde::de::Error::custom("Failed to parse value into u64"))
        },
        Err(err) => Err(err)
    }
}