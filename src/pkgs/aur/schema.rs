use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::utils::epocher::deserialize_unix_timestamp;
use chrono::DateTime;
use crate::pkgs::std::Arch;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    pub version: i32,
    #[serde(rename = "type")]
    pub types: Type,
    #[serde(rename = "resultcount")]
    pub result_count: i32,
    pub results: Vec<Data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Search,
    Error,
    MultiInfo,
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
    pub description: Option<String>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "NumVotes")]
    pub num_votes: i32,
    #[serde(rename = "Popularity")]
    pub popularity: f64,
    #[serde(rename = "OutOfDate")]
    pub out_of_date: Option<i32>,
    #[serde(rename = "Maintainer")]
    pub maintainer: Option<String>,
    #[serde(rename = "FirstSubmitted", deserialize_with = "deserialize_unix_timestamp")]
    pub first_submitted: DateTime<chrono::Utc>,
    #[serde(rename = "LastModified", deserialize_with = "deserialize_unix_timestamp")]
    pub last_modified: DateTime<chrono::Utc>,
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

impl crate::pkgs::any::ToAny for Data {
    fn to_any(&self) -> crate::pkgs::any::Data {
        crate::pkgs::any::Data {
            name: self.name.clone(),
            base: self.package_base.clone(),
            description: self.description.clone(),
            arch: Arch::X86_64,
            repo: None,
            types: crate::pkgs::any::Type::Aur,
            version: self.version.clone(),
            url: self.version.clone(),
            author: match self.maintainer.clone() {
                Some(maintainer) => vec![maintainer],
                None => vec![],
            },
            updated: self.last_modified,
            install: format!("paru -S {}", self.name),
        }
    }
}