use serde::{Deserialize, Serialize};
use std::fmt::Display;
use chrono::DateTime;
use crate::pkgs::any::{ToAny, Type};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    pub version: i32,
    pub limit: i32,
    pub valid: bool,
    pub results: Vec<Data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_pages: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    #[serde(rename = "pkgname")]
    pub pkg_name: String,
    #[serde(rename = "pkgbase")]
    pub pkg_base: String,
    #[serde(rename = "repo")]
    pub repo: Repo,
    #[serde(rename = "arch")]
    pub arch: Arch,
    #[serde(rename = "pkgver")]
    pub pkg_ver: String,
    #[serde(rename = "pkgrel")]
    pub pkg_rel: String,
    #[serde(rename = "epoch")]
    pub epoch: i32,
    #[serde(rename = "pkgdesc")]
    pub pkg_desc: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "filename")]
    pub filename: String,
    pub compressed_size: u64,
    pub installed_size: u64,
    pub build_date: DateTime<chrono::Utc>,
    pub last_update: DateTime<chrono::Utc>,
    pub flag_date: Option<String>,
    pub maintainers: Vec<String>,
    pub packager: String,
    pub groups: Vec<String>,
    pub licenses: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<String>,
    pub depends: Vec<String>,
    #[serde(rename = "optdepends")]
    pub opt_depends: Vec<String>,
    #[serde(rename = "makedepends")]
    pub make_depends: Vec<String>,
    #[serde(rename = "checkdepends")]
    pub check_depends: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Repo {
    // Core Family
    Core,
    #[serde(rename = "core-testing")]
    CoreTesting,

    // Extra Family
    Extra,
    #[serde(rename = "extra-testing")]
    ExtraTesting,

    // Multilib Family
    Multilib,
    #[serde(rename = "multilib-testing")]
    MultilibTesting,

    // DE Family
    #[serde(rename = "gnome-unstable")]
    GnomeUnstable,
    #[serde(rename = "kde-unstable")]
    KdeUnstable,

    // Community Family
    Testing,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
    Any,
    X86_64,
    X86,
}

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Repo::Core => "core".to_string(),
            Repo::CoreTesting => "core-testing".to_string(),
            Repo::Extra => "extra".to_string(),
            Repo::ExtraTesting => "extra-testing".to_string(),
            Repo::Multilib => "multilib".to_string(),
            Repo::MultilibTesting => "multilib-testing".to_string(),
            Repo::GnomeUnstable => "gnome-unstable".to_string(),
            Repo::KdeUnstable => "kde-unstable".to_string(),
            Repo::Testing => "testing".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Arch::Any => "any".to_string(),
            Arch::X86_64 => "x86_64".to_string(),
            Arch::X86 => "i686".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl ToAny for Data {
    fn to_any(&self) -> crate::pkgs::any::Data {
        crate::pkgs::any::Data {
            name: self.pkg_name.clone(),
            base: self.pkg_base.clone(),
            description: Some(self.pkg_desc.clone()),
            arch: self.arch.clone(),
            repo: Some(self.repo.clone()),
            types: Type::Std,
            version: self.pkg_ver.clone(),
            url: self.url.clone(),
            author: self.maintainers.clone(),
            updated: self.last_update,
            install: format!("sudo pacman -S {}", self.pkg_name.clone()),
        }
    }
}