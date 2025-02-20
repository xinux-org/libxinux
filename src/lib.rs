//! # Libxinux
//!
//! This is a library containing various utilities for interacting with the Arch Linux ecosystem
//! that will be used to create various tools for Xinux'es personal use.
//!
//! ## Features
//! We provide blocking features by default. However, you can enable async features if you want to.
//!
//! Default features:
//! - `pkgs`: Enables the `pkgs` module which contains utilities for interacting with the package
//!
//! Optional features:
//! - `pkgs-async`: Enables the `pkgs` module with async support
//!
//! ## Overview
//!
//! ### Packages (`pkgs` module)
//!
//! The `pkgs` module contains apis for fetching package information from the Arch Linux
//! package repositories and the Arch User Repository (AUR). Also, it's possible to fetch group
//! data from the Arch Linux package repositories. We used **tokio** and **reqwest** to create async
//! versions of the apis.

#![allow(clippy::inherent_to_string)]

use anyhow::Result;
use std::{fs, process::Command};

pub mod config;
pub mod homemanager;
pub mod metadata;
pub mod nixenv;
pub mod nixos;
pub mod profile;
pub mod utils;

#[derive(Debug, Clone, Default)]
pub struct Package {
    pub attr: PackageAttr,
    pub pname: Option<String>,
    pub version: Option<String>,

    pub profile_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PackageUpdate {
    pub attr: PackageAttr,
    pub new_version: String,
    pub old_version: String,
}

#[derive(Debug, Clone)]
pub enum PackageAttr {
    NixPkgs { attr: String },
    External { url: String, attr: String },
}

impl PackageAttr {
    pub fn to_string(&self) -> String {
        match self {
            PackageAttr::NixPkgs { attr } => attr.to_string(),
            PackageAttr::External { url, attr } => format!("{}#{}", url, attr),
        }
    }
}

impl Default for PackageAttr {
    fn default() -> Self {
        PackageAttr::NixPkgs {
            attr: String::new(),
        }
    }
}

pub fn get_nix_arch() -> Result<String> {
    let output = Command::new("nix")
        .arg("--experimental-features")
        .arg("nix-command")
        .arg("show-config")
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let arch = stdout
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .find(|x| x.contains("system ="))
        .unwrap()
        .split("=")
        .collect::<Vec<_>>()[1]
        .trim()
        .to_string();
    Ok(arch.to_string())
}

pub fn get_nixos_arch() -> Result<String> {
    let output = fs::read_to_string("/run/current-system/system")?;
    Ok(output)
}

pub fn get_eval_arch() -> Result<String> {
    let output = Command::new("nix")
        .arg("--experimental-features")
        .arg("nix-command flakes")
        .arg("eval")
        .arg("nixpkgs#system")
        .arg("--raw")
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout.to_string())
}

fn get_arch() -> String {
    if let Ok(arch) = get_nixos_arch() {
        return arch;
    }
    if let Ok(arch) = get_nix_arch() {
        return arch;
    }
    if let Ok(arch) = get_eval_arch() {
        return arch;
    }
    panic!("Could not determine architecture");
}

lazy_static::lazy_static! {
    pub static ref NIXARCH: String = get_arch();
    pub static ref CACHEDIR: String = format!("{}/.cache/libxinux", std::env::var("HOME").unwrap());
    pub static ref CONFIGDIR: String = format!("{}/.config/libxinux", std::env::var("HOME").unwrap());
    pub static ref CONFIG: String = format!("{}/config.json", &*CONFIGDIR);
    pub static ref HOME: String = std::env::var("HOME").unwrap();
    pub static ref IS_NIXOS: bool = std::path::Path::new("/etc/NIXOS").exists();
}
static SYSCONFIG: &str = "/etc/libxinux/config.json";
static HELPER_EXEC: &str = "libxinux-helper";
static ICON_UPDATER_EXEC: &str = "update-icons.trigger";
