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

/// Contains utilities for interacting with the package
#[cfg(any(feature = "pkgs", feature = "pkgs-async"))]
pub mod pkgs;

/// Contains error types for the library
pub mod error;

/// Contains utilities and implementations for the library
pub mod utils;
