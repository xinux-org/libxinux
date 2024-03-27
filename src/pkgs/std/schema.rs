use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Response {
    version: i32,
    limit: i32,
    valid: bool,
    results: Vec<Data>,
    num_pages: i32,
    page: i32

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Data {
    #[serde(rename = "pkgname")]
    pkg_name: String,
    #[serde(rename = "pkgbase")]
    pkg_base: String,
    #[serde(rename = "repo")]
    repo: String,
    #[serde(rename = "arch")]
    arch: Arch,
    #[serde(rename = "pkgver")]
    pkg_ver: String,
    #[serde(rename = "pkgrel")]
    pkg_rel: String,
    #[serde(rename = "epoch")]
    epoch: i32,
    #[serde(rename = "pkgdesc")]
    pkg_desc: String,
    #[serde(rename = "url")]
    url: String,
    #[serde(rename = "filename")]
    filename: String,
    compressed_size: i32,
    installed_size: i32,
    build_date: String,
    last_update: String,
    flag_date: String,
    maintainers: Vec<String>,
    packager: Vec<String>,
    groups: Vec<String>,
    licenses: Vec<String>,
    conflicts: Vec<String>,
    provides: Vec<String>,
    replaces: Vec<String>,
    depends: Vec<String>,
    #[serde(rename = "optdepends")]
    opt_depends: Vec<String>,
    #[serde(rename = "makedepends")]
    make_depends: Vec<String>,
    #[serde(rename = "checkdepends")]
    check_depends: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Repo {
    Core,
    Extra,
    Community,
    Testing,
    Multilib,
    #[serde(rename = "multilib-testing")]
    MultilibTesting,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
    Any,
    X86_64,
    X86
}