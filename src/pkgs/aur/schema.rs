use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    pub version: i32,
    #[serde(rename = "type")]
    pub types: String,
    #[serde(rename = "resultcount")]
    pub result_count: i32,
    pub results: Vec<Data>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PackageBaseID")]
    pub package_base_id: i64,
    #[serde(rename = "PackageBase")]
    pub package_base: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "NumVotes")]
    pub num_votes: i32,
    #[serde(rename = "Popularity")]
    pub popularity: f64,
    #[serde(rename = "OutOfDate")]
    pub out_of_date: Option<i32>,
    #[serde(rename = "Maintainer")]
    pub maintainer: Option<String>,
    #[serde(rename = "FirstSubmitted")]
    pub first_submitted: i32,
    #[serde(rename = "LastModified")]
    pub last_modified: i32,
    #[serde(rename = "URLPath")]
    pub url_path: String,
    #[serde(rename = "Depends")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends: Option<Vec<String>>,
    #[serde(rename = "License")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<Vec<String>>,
    #[serde(rename = "Keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

pub enum By {
    Name,
    NameDesc,
    Maintainer,
    Depends,
    MakeDepends,
    OptDepends,
    CheckDepends,
}

impl Display for By {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            By::Name => "name".to_string(),
            By::NameDesc => "name-desc".to_string(),
            By::Maintainer => "maintainer".to_string(),
            By::Depends => "depends".to_string(),
            By::MakeDepends => "make-depends".to_string(),
            By::OptDepends => "opt-depends".to_string(),
            By::CheckDepends => "check-depends".to_string(),
        };
        write!(f, "{}", str)
    }
}
