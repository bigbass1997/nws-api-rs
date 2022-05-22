use std::cmp::{max, min};
use chrono::{DateTime, Utc};
use reqwest::blocking::{Client as ReqClient, ClientBuilder, Response};
use reqwest::Error;
use serde::de::DeserializeOwned;
use crate::endpoints::gridpoints::{Gridpoint, GridpointStations};
use crate::endpoints::points::Point;
use crate::endpoints::radar::{crawl, RadarType, RemoteFile};
use crate::endpoints::stations::{Observation, ObservationCollection, ObservationStation, ObservationStationCollection};

pub mod endpoints;


pub const ROOT: &'static str = "https://api.weather.gov";
pub const DATETIME_FMT: &'static str = "%Y-%m-%dT%H:%M:%S%.3f%:z";

#[derive(Debug)]
pub enum NwsError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
}
impl From<reqwest::Error> for NwsError {
    fn from(err: Error) -> Self {
        NwsError::Reqwest(err)
    }
}
impl From<serde_json::Error> for NwsError {
    fn from(err: serde_json::Error) -> Self {
        NwsError::Serde(err)
    }
}

pub type Result<T> = std::result::Result<T, NwsError>;

/// Client used to access the NWS API endpoints.
/// 
/// All functions are blocking.
#[derive(Debug)]
pub struct NwsClient {
    pub req: ReqClient,
}
impl NwsClient {
    pub fn new(user_agent: &str) -> Result<NwsClient> {
        match ClientBuilder::new().user_agent(user_agent).build() {
            Ok(req) => Ok(NwsClient {
                req
            }),
            Err(err) => Err(err.into())
        }
    }
    
    pub fn gridpoints(&self, id: &str, x: usize, y: usize) -> Result<Gridpoint> {
        parse_result(get(&self.req, &format!("/gridpoints/{}/{},{}", id, x, y), None))
    }
    
    pub fn gridpoints_stations(&self, id: &str, x: usize, y: usize) -> Result<GridpointStations> {
        parse_result(get(&self.req, &format!("/gridpoints/{}/{},{}/stations", id, x, y), None))
    }
    
    pub fn stations_observations(&self, id: &str, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>, limit: Option<usize>) -> Result<ObservationCollection> {
        let mut params = Vec::new();
        if let Some(start) = start { params.push(("start", start.format(DATETIME_FMT).to_string())); }
        if let Some(end) = end { params.push(("end", end.format(DATETIME_FMT).to_string())); }
        if let Some(limit) = limit { params.push(("limit", max(1, min(500, limit)).to_string())); }
        
        parse_result(get(&self.req, &format!("/stations/{}/observations", id), Some(params)))
    }
    
    pub fn stations_observations_latest(&self, id: &str, require_qc: Option<bool>) -> Result<Observation> {
        let mut params = None;
        if let Some(require_qc) = require_qc {
            params = Some(vec![("require_qc", require_qc.to_string())]);
        }
        
        parse_result(get(&self.req, &format!("/stations/{}/observations/latest", id), params))
    }
    
    pub fn stations_observations_time(&self, id: &str, time: DateTime<Utc>) -> Result<Observation> {
        let time = time.format(DATETIME_FMT).to_string();
        
        parse_result(get(&self.req, &format!("/stations/{}/observations/{}", id, time), None))
    }
    
    pub fn stations(&self, ids: Option<&[&str]>, states: Option<&[&str]>, limit: Option<usize>) -> Result<ObservationStationCollection> {
        let mut params = Vec::new();
        if let Some(ids) = ids { params.push(("id", ids.join(","))); }
        if let Some(states) = states { params.push(("state", states.join(","))); }
        if let Some(limit) = limit { params.push(("limit", max(1, min(500, limit)).to_string())); }
        
        parse_result(get(&self.req, "/stations", Some(params)))
    }
    
    pub fn stations_id(&self, id: &str) -> Result<ObservationStation> {
        parse_result(get(&self.req, &format!("/stations/{}", id), None))
    }
    
    pub fn points(&self, lat: f64, lon: f64) -> Result<Point> {
        parse_result(get(&self.req, &format!("/points/{:.4},{:.4}", lat, lon), None))
    }
    
    
    pub fn radar(&self, id: &str, radar_type: RadarType) -> Result<Vec<RemoteFile>> {
        crawl(&self.req, format!("https://mrms.ncep.noaa.gov/data/RIDGEII/L2/{}/{}/", id, radar_type))
    }
}

fn get(req: &ReqClient, endpoint: &str, params: Option<Vec<(&str, String)>>) -> Result<Response> {
    let mut req = req.get(format!("{}{}", ROOT, endpoint))
        .header("Content-Type", "application/ld+json")
        .header("Accept", "application/ld+json");
    if let Some(params) = params {
        req = req.query(&params);
    }
    
    match req.send() {
        Ok(resp) => Ok(resp),
        Err(err) => Err(err.into())
    }
}

fn parse_result<T: DeserializeOwned>(resp: Result<Response>) -> Result<T> {
    let resp = match resp {
        Ok(resp) => match resp.text() {
            Ok(text) => text,
            Err(err) => return Err(err.into())
        },
        Err(err) => return Err(err)
    };
    
    match serde_json::from_str(&resp) {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.into())
    }
}