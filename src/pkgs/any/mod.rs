//! # Any Module
//!
//! Fetch hybrid data from both standard and AUR repositories.

pub mod schema;

use super::{aur::Aur, std::Std};
use crate::error::Result;
pub use schema::*;
use serde::{Deserialize, Serialize};

/// Convert a struct to Any's `Data` struct.
pub trait ToAny {
    fn to_any(&self) -> Data;
}

/// Any struct for fetching hybrid data from both standard and AUR repositories.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Any {
    /// Standard repository client.
    pub std: Std,

    /// AUR repository client.
    pub aur: Aur,
}

impl Any {
    /// Create a new instance of `Any`.
    pub fn new() -> Result<Any> {
        Ok(Any {
            std: match Std::new() {
                Ok(std) => std,
                Err(e) => return Err(e),
            },
            aur: match Aur::new() {
                Ok(aur) => aur,
                Err(e) => return Err(e),
            },
        })
    }

    /// Returns a vector of `Data` structs that belongs to Any struct.
    ///
    /// Search for a package in both standard and AUR repositories with blocking support.
    ///
    /// # Example
    /// ```no_run
    /// use libxinux::pkgs::any::Any;
    /// use libxinux::error::Result;
    ///
    /// let any = Any::new().unwrap();
    /// let response = any.search("linux").unwrap();
    /// assert_eq!(response.first().unwrap().name, "linux");
    /// ```
    #[cfg(not(feature = "pkgs-async"))]
    pub fn search<T>(&self, query: T) -> Result<Vec<Data>>
    where
        T: AsRef<str> + Clone,
    {
        let mut data: Vec<Data> = Vec::new();

        match self.std.search(query.clone()) {
            Ok(std) => std.results.iter().for_each(|item| {
                data.push(item.to_any());
            }),
            Err(e) => return Err(e),
        };

        match self.aur.search(query.clone(), None) {
            Ok(aur) => aur.results.iter().for_each(|item| {
                data.push(item.to_any());
            }),
            Err(e) => return Err(e),
        };

        let results = Data::fuzzy_search(data, query)
            .iter()
            .map(|item| item.0.clone())
            .collect();

        Ok(results)
    }

    /// Returns a vector of `Data` structs that belongs to Any struct.
    ///
    /// Search for a package in both standard and AUR repositories with async support.
    ///
    /// # Example
    /// ```no_run
    /// use libxinux::pkgs::any::Any;
    /// use libxinux::error::Result;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let any = Any::new().unwrap();
    ///     let response = any.search("linux").await.unwrap();
    ///     assert_eq!(response.first().unwrap().name, "linux");
    /// }
    /// ```
    #[cfg(feature = "pkgs-async")]
    pub async fn search<T>(&self, query: T) -> Result<Vec<Data>>
    where
        T: AsRef<str> + Clone,
    {
        let mut data: Vec<Data> = Vec::new();

        match self.std.search(query.clone()).await {
            Ok(std) => std.results.iter().for_each(|item| {
                data.push(item.to_any());
            }),
            Err(e) => return Err(e),
        };

        match self.aur.search(query.clone(), None).await {
            Ok(aur) => aur.results.iter().for_each(|item| {
                data.push(item.to_any());
            }),
            Err(e) => return Err(e),
        };

        let results = Data::fuzzy_search(data, query)
            .iter()
            .map(|item| item.0.clone())
            .collect();

        Ok(results)
    }

    /// Returns a `Data` struct that belongs to Any struct.
    ///
    /// Fetch information about a package in both standard and AUR repositories with blocking support.
    ///
    /// # Example
    /// ```no_run
    /// use libxinux::pkgs::any::Any;
    /// use libxinux::error::Result;
    ///
    /// let any = Any::new().unwrap();
    /// let response = any.info("linux").unwrap();
    /// assert_eq!(response.name, "linux");
    /// ```
    #[cfg(not(feature = "pkgs-async"))]
    pub fn info<T>(&self, query: T) -> Result<Data>
    where
        T: AsRef<str> + Clone,
    {
        let data = match self.search(query.clone()) {
            Ok(data) => data.first().unwrap().clone(),
            Err(e) => return Err(e),
        };

        let result = match data.types {
            Type::Std => {
                let action = self.std.info(query.as_ref(), data.repo.unwrap());

                match action {
                    Ok(action) => action.to_any(),
                    Err(e) => return Err(e),
                }
            }
            Type::Aur => {
                let action = self.aur.info(query.as_ref());

                match action {
                    Ok(action) => action.to_any(),
                    Err(e) => return Err(e),
                }
            }
        };

        Ok(result)
    }

    /// Returns a `Data` struct that belongs to Any struct.
    ///
    /// Fetch information about a package in both standard and AUR repositories with async support.
    ///
    /// # Example
    /// ```no_run
    /// use libxinux::pkgs::any::Any;
    /// use libxinux::error::Result;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let any = Any::new().unwrap();
    ///     let response = any.info("linux").await.unwrap();
    ///     assert_eq!(response.name, "linux");
    /// }
    /// ```
    #[cfg(feature = "pkgs-async")]
    pub async fn info<T>(&self, query: T) -> Result<Data>
    where
        T: AsRef<str> + Clone,
    {
        let data = match self.search(query.clone()).await {
            Ok(data) => data.first().unwrap().clone(),
            Err(e) => return Err(e),
        };

        let result = match data.types {
            Type::Std => {
                let action = self.std.info(query.as_ref(), data.repo.unwrap()).await;

                match action {
                    Ok(action) => action.to_any(),
                    Err(e) => return Err(e),
                }
            }
            Type::Aur => {
                let action = self.aur.info(query.as_ref()).await;

                match action {
                    Ok(action) => action.to_any(),
                    Err(e) => return Err(e),
                }
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        println!("any_test_new");
        let any = Any::new().unwrap();

        assert_eq!(any.std.url.as_str(), "https://archlinux.org/packages/");
        assert_eq!(any.aur.url.as_str(), "https://aur.archlinux.org/rpc/?v=5");
    }

    #[test]
    #[cfg(not(feature = "pkgs-async"))]
    fn test_search() {
        println!("any_test_search");
        let any = Any::new().unwrap();
        let response = any.search("linux").unwrap();

        assert_eq!(response.first().unwrap().name, "linux");
    }

    #[tokio::test]
    #[cfg(feature = "pkgs-async")]
    async fn test_search_async() {
        println!("any_test_search_async");
        let any = Any::new().unwrap();
        let response = any.search("linux").await.unwrap();

        assert_eq!(response.first().unwrap().name, "linux");
    }

    #[test]
    #[cfg(not(feature = "pkgs-async"))]
    fn test_info() {
        println!("any_test_info");
        let any = Any::new().unwrap();
        let response = any.info("linux").unwrap();

        assert_eq!(response.name, "linux");
    }

    #[tokio::test]
    #[cfg(feature = "pkgs-async")]
    async fn test_info_async() {
        println!("any_test_info_async");
        let any = Any::new().unwrap();
        let response = any.info("linux").await.unwrap();

        assert_eq!(response.name, "linux");
    }
}
