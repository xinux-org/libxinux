//! # AUR (Arch User Repository) module
//!
//! Fetch information about packages from the AUR (Arch User Repository).

pub mod schema;

use crate::error::{Error, Result};
pub use schema::*;
use serde::{Deserialize, Serialize};
use url::Url;

/// Base RPC URL for the AUR API.
const BASE_URL: &str = "https://aur.archlinux.org/rpc/?v=5";

/// Aur struct for fetching information about packages from the AUR.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aur {
    /// Base URL for the AUR API.
    pub(crate) url: Url,
}

impl Aur {
    /// Create a new Aur instance.
    pub fn new() -> Result<Aur> {
        let url = match Url::parse(BASE_URL) {
            Ok(url) => url,
            Err(e) => return Err(e.into()),
        };

        Ok(Aur { url })
    }

    /// Returns nothing.
    ///
    /// Set the base URL for the AUR API.
    ///
    /// # Example
    /// ```no_run
    /// use libxinux::pkgs::aur::Aur;
    /// use libxinux::error::Result;
    ///
    /// let mut aur = Aur::new().unwrap();
    /// aur.set_url("https://aur.archlinux.org/rpc/?v=5".to_string()).unwrap();
    /// ```
    pub fn set_url(&mut self, base_url: String) -> Result<()> {
        let url = Url::parse(&base_url);

        let url = match url {
            Ok(url) => url,
            Err(e) => return Err(e.into()),
        };

        self.url = url;

        Ok(())
    }

    /// Returns a `Response` struct that belongs to Aur struct.
    ///
    /// Search for a package in the AUR repository with blocking support.
    ///
    /// # Example
    /// ```no_run
    /// use libxinux::pkgs::aur::Aur;
    /// use libxinux::error::Result;
    ///
    /// let aur = Aur::new().unwrap();
    /// let response = aur.search("archlinux-hello", None).unwrap();
    /// assert_eq!(response.results.first().unwrap().id, 1193389);
    /// ```
    #[cfg(not(feature = "pkgs-async"))]
    pub fn search<T>(&self, query: T, by: Option<By>) -> Result<Response>
    where
        T: AsRef<str>,
    {
        let url = format!("{}&type=search&arg={}", self.url.as_str(), query.as_ref(),);

        let url = match by {
            Some(by) => format!("{}&by={}", url, by),
            None => url,
        };

        let response = match reqwest::blocking::get(url) {
            Ok(response) => response.json::<Response>(),
            Err(e) => return Err(e.into()),
        };

        let response = match response {
            Ok(response) => response,
            Err(e) => return Err(e.into()),
        };

        if response.error.is_some() {
            return Err(Error::ResponseError(response.error.unwrap()));
        }

        Ok(response)
    }

    /// Returns a `Response` struct that belongs to Aur struct.
    ///
    /// Search for a package in the AUR repository with async support.
    ///
    /// # Example
    /// ```no_run
    /// use libxinux::pkgs::aur::Aur;
    /// use libxinux::error::Result;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let aur = Aur::new().unwrap();
    ///     let response = aur.search("archlinux-hello", None).await.unwrap();
    ///     assert_eq!(response.results.first().unwrap().id, 1193389);
    /// }
    /// ```
    #[cfg(feature = "pkgs-async")]
    pub async fn search<T>(&self, query: T, by: Option<By>) -> Result<Response>
    where
        T: AsRef<str>,
    {
        let url = format!("{}&type=search&arg={}", self.url.as_str(), query.as_ref(),);

        let url = match by {
            Some(by) => format!("{}&by={}", url, by),
            None => url,
        };

        let response = match reqwest::get(url).await {
            Ok(response) => response.json::<Response>(),
            Err(e) => return Err(e.into()),
        };

        let response = match response.await {
            Ok(response) => response,
            Err(e) => return Err(e.into()),
        };

        if response.error.is_some() {
            return Err(Error::ResponseError(response.error.unwrap()));
        }

        Ok(response)
    }

    /// Returns a `Data` struct that belongs to Aur struct.
    ///
    /// Fetch information about a package in the AUR repository with blocking support.
    ///
    /// # Example
    /// ```no_run
    /// use libxinux::pkgs::aur::Aur;
    /// use libxinux::error::Result;
    ///
    /// let aur = Aur::new().unwrap();
    /// let response = aur.info("archlinux-hello").unwrap();
    /// assert_eq!(response.id, 1193389);
    /// ```
    #[cfg(not(feature = "pkgs-async"))]
    pub fn info(&self, name: &str) -> Result<Data> {
        let url = format!("{}&type=info&arg[]={}", self.url.as_str(), name);

        let response = match reqwest::blocking::get(url) {
            Ok(response) => response.json::<Response>(),
            Err(e) => return Err(e.into()),
        };

        let response = match response {
            Ok(response) => response,
            Err(e) => return Err(e.into()),
        };

        if response.error.is_some() {
            return Err(Error::ResponseError(response.error.unwrap()));
        }

        if response.result_count == 0 {
            return Err(Error::NoResults(name.to_string()));
        }

        Ok(response.results.first().unwrap().clone())
    }

    /// Returns a `Data` struct that belongs to Aur struct.
    ///
    /// Fetch information about a package in the AUR repository with async support.
    ///
    /// # Example
    /// ```no_run
    /// use libxinux::pkgs::aur::Aur;
    /// use libxinux::error::Result;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let aur = Aur::new().unwrap();
    ///     let response = aur.info("archlinux-hello").await.unwrap();
    ///     assert_eq!(response.id, 1193389);
    /// }
    /// ```
    #[cfg(feature = "pkgs-async")]
    pub async fn info(&self, name: &str) -> Result<Data> {
        let url = format!("{}&type=info&arg[]={}", self.url.as_str(), name);

        let response = match reqwest::get(url).await {
            Ok(response) => response.json::<Response>(),
            Err(e) => return Err(e.into()),
        };

        let response = match response.await {
            Ok(response) => response,
            Err(e) => return Err(e.into()),
        };

        if response.error.is_some() {
            return Err(Error::ResponseError(response.error.unwrap()));
        }

        if response.result_count == 0 {
            return Err(Error::NoResults(name.to_string()));
        }

        Ok(response.results.first().unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        println!("aur_test_new");
        let aur = Aur::new().unwrap();
        assert_eq!(aur.url.as_str(), BASE_URL);
    }

    #[test]
    #[cfg(not(feature = "pkgs-async"))]
    fn test_search() {
        println!("aur_test_search");
        let aur = Aur::new().unwrap();
        let response = aur.search("archlinux-hello", None).unwrap();

        assert_eq!(response.results.first().unwrap().id, 1193389);
    }

    #[tokio::test]
    #[cfg(feature = "pkgs-async")]
    async fn test_search_async() {
        println!("aur_test_search_async");
        let aur = Aur::new().unwrap();
        let response = aur.search("archlinux-hello", None).await.unwrap();

        assert_eq!(response.results.first().unwrap().id, 1193389);
    }

    #[test]
    #[cfg(not(feature = "pkgs-async"))]
    fn test_info() {
        println!("aur_test_info");
        let aur = Aur::new().unwrap();
        let response = aur.info("archlinux-hello").unwrap();

        assert_eq!(response.id, 1193389);
    }

    #[tokio::test]
    #[cfg(feature = "pkgs-async")]
    async fn test_info_async() {
        println!("aur_test_info_async");
        let aur = Aur::new().unwrap();
        let response = aur.info("archlinux-hello").await.unwrap();

        assert_eq!(response.id, 1193389);
    }
}
