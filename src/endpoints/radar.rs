use std::fmt::{Display, Formatter};
use chrono::{DateTime, NaiveDateTime, Utc};
use select::document::Document;
use select::node::Node;
use select::predicate::{Child, Name};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Unexpected, Visitor};
use url::Url;
use crate::{NwsError, ReqClient};


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RadarType {
    BrefRaw,
    BvelRaw,
    
    Bdhc,
    Bdsa,
    Bdzd,
    Beet,
    Bohp,
    Bref,
    Bsrm,
    Bsta,
    Bstp,
    Bvel,
    Cref,
    Hvil,
}
impl Display for RadarType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use RadarType::*;
        write!(f, "{}", match self {
            BrefRaw => "BREF_RAW",
            BvelRaw => "BVEL_RAW",
            Bdhc => "BDHC",
            Bdsa => "BDSA",
            Bdzd => "BDZD",
            Beet => "BEET",
            Bohp => "BOHP",
            Bref => "BREF",
            Bsrm => "BSRM",
            Bsta => "BSTA",
            Bstp => "BSTP",
            Bvel => "BVEL",
            Cref => "CREF",
            Hvil => "HVIL",
        })
    }
}
impl Serialize for RadarType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for RadarType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct RadarTypeVisitor;
        impl<'de> Visitor<'de> for RadarTypeVisitor {
            type Value = RadarType;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A string matching the Display impl for RadarType.")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
                use RadarType::*;
                match v {
                    "BREF_RAW" => Ok(BrefRaw),
                    "BVEL_RAW" => Ok(BvelRaw),
                    "BDHC" => Ok(Bdhc),
                    "BDSA" => Ok(Bdsa),
                    "BDZD" => Ok(Bdzd),
                    "BEET" => Ok(Beet),
                    "BOHP" => Ok(Bohp),
                    "BREF" => Ok(Bref),
                    "BSRM" => Ok(Bsrm),
                    "BSTA" => Ok(Bsta),
                    "BSTP" => Ok(Bstp),
                    "BVEL" => Ok(Bvel),
                    "CREF" => Ok(Cref),
                    "HVIL" => Ok(Hvil),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(v), &"RadarType variant Display value"))
                }
            }
        }
        
        deserializer.deserialize_identifier(RadarTypeVisitor)
    }
}
impl RadarType {
    pub fn values() -> Vec<RadarType> {
        use RadarType::*;
        vec![BrefRaw, BvelRaw, Bdhc, Bdsa, Bdzd, Beet, Bohp, Bref, Bsrm, Bsta, Bstp, Bvel, Cref, Hvil]
    }
}


#[derive(Copy, Clone, Debug)]
pub enum FileSize {
    B(f64),
    K(f64),
    M(f64),
    G(f64),
}
impl FileSize {
    pub fn new(value: impl AsRef<str>) -> Option<Self> {
        let value = value.as_ref();
        if value.is_empty() || value == "-" { return None; }
        
        let last = value.chars().last().unwrap();
        let num: f64 = if last.is_numeric() {
            value.parse().unwrap()
        } else {
            value.split_at(value.len() - 1).0.parse().unwrap()
        };
        
        use FileSize::*;
        match last {
            'K' => Some(K(num)),
            'M' => Some(M(num)),
            'G' => Some(G(num)),
            
            _ if last.is_numeric() => Some(B(num)),
            _ => None
        }
    }
}

#[derive(Clone, Debug)]
pub struct RemoteFile {
    pub url: String,
    pub last_modified: Option<DateTime<Utc>>,
    pub size: Option<FileSize>,
}
impl RemoteFile {
    pub fn new(url: impl AsRef<str>, last_modified: Option<DateTime<Utc>>, size: Option<FileSize>) -> Self { Self {
        url: url.as_ref().to_owned(),
        last_modified,
        size,
    }}
}

pub(crate) fn crawl(req: &ReqClient, url: impl AsRef<str>) -> Result<Vec<RemoteFile>, NwsError> {
    let url = url.as_ref();
    let mut files = vec![];
    
    let resp = match req.get(url).send() {
        Ok(resp) => resp,
        Err(err) => return Err(err.into())
    };
    let html = resp.text().unwrap();
    let doc = Document::from(html.as_ref());
    
    let select: Vec<Node> = doc.select(Child(Name("tr"), Name("td"))).collect();
    for i in (0..select.len()).step_by(3) {
        let name = select[i].text().trim().to_owned();
        let modified = select[i + 1].text().trim().to_owned();
        let size = select[i + 2].text().trim().to_owned();
        
        let url = if name == "Parent Directory" && size == "-" {
            let url = Url::parse(url).unwrap();
            format!("{}://{}{}", url.scheme(), url.host_str().unwrap(), select[i].first_child().unwrap().attr("href").unwrap())
        } else {
            format!("{}{}", url, name)
        };
        
        let modified = if modified.is_empty() {
            None
        } else {
            Some(DateTime::from_utc(NaiveDateTime::parse_from_str(&modified, "%d-%b-%Y %H:%M").unwrap(), Utc))
        };
        
        files.push(RemoteFile::new(url, modified, FileSize::new(size)));
    }
    
    Ok(files)
}