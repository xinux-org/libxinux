pub mod schema;

use crate::error::{Error, Result};
pub use schema::*;
use serde::{Deserialize, Serialize};
use url::Url;

const BASE_URL: &str = "https://archlinux.org/packages/"; // /core/x86_64/linux/json

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Std {
    pub(crate) url: Url,
}

impl Std {
    pub fn new() -> Result<Std> {
        let url = match Url::parse(BASE_URL) {
            Ok(url) => url,
            Err(e) => return Err(e.into()),
        };

        Ok(Std { url })
    }

    pub fn set_url(&mut self, base_url: String) -> Result<()> {
        let url = Url::parse(&base_url);

        let url = match url {
            Ok(url) => url,
            Err(e) => return Err(e.into()),
        };

        self.url = url;

        Ok(())
    }

    #[cfg(not(feature = "pkgs-async"))]
    pub fn search<T>(&self, query: T) -> Result<Response>
    where
        T: AsRef<str>,
    {
        let url = format!("{}search/json/?q={}", self.url.as_str(), query.as_ref(),);

        let response = match reqwest::blocking::get(url) {
            Ok(response) => response.json::<Response>(),
            Err(e) => return Err(e.into()),
        };

        let response = match response {
            Ok(response) => response,
            Err(e) => return Err(e.into()),
        };

        Ok(response)
    }

    #[cfg(feature = "pkgs-async")]
    pub async fn search<T>(&self, query: T) -> Result<Response>
    where
        T: AsRef<str>,
    {
        let url = format!("{}search/json/?q={}", self.url.as_str(), query.as_ref(),);

        let response = match reqwest::get(url).await {
            Ok(response) => response.json::<Response>().await,
            Err(e) => return Err(e.into()),
        };

        let response = match response {
            Ok(response) => response,
            Err(e) => return Err(e.into()),
        };

        Ok(response)
    }

    #[cfg(not(feature = "pkgs-async"))]
    pub fn info<T>(&self, name: T, repo: Repo) -> Result<Data>
    where
        T: AsRef<str> + std::fmt::Display,
    {
        let url = format!(
            "{base}/{repo}/{arch}/{name}/json",
            base = self.url.as_str(),
            repo = repo,
            arch = Arch::X86_64,
            name = name
        );

        let response = match reqwest::blocking::get(url) {
            Ok(response) => response.json::<Data>(),
            Err(e) => return Err(e.into()),
        };

        let response = match response {
            Ok(response) => response,
            Err(_) => return Err(Error::NoResults(name.to_string())),
        };

        Ok(response)
    }

    #[cfg(feature = "pkgs-async")]
    pub async fn info<T>(&self, name: T, repo: Repo) -> Result<Data>
    where
        T: AsRef<str> + std::fmt::Display,
    {
        let url = format!(
            "{base}/{repo}/{arch}/{name}/json",
            base = self.url.as_str(),
            repo = repo,
            arch = Arch::X86_64,
            name = name
        );

        let response = match reqwest::get(url).await {
            Ok(response) => response.json::<Data>().await,
            Err(e) => return Err(e.into()),
        };

        let response = match response {
            Ok(response) => response,
            Err(_) => return Err(Error::NoResults(name.to_string())),
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        println!("std_test_new");
        let std = Std::new().unwrap();
        assert_eq!(std.url.as_str(), BASE_URL);
    }

    #[test]
    #[cfg(not(feature = "pkgs-async"))]
    fn test_search() {
        println!("std_test_search");
        let std = Std::new().unwrap();
        let response = std.search("linux").unwrap();

        assert_eq!(
            response.results.first().unwrap().pkg_name,
            "aarch64-linux-gnu-binutils"
        );
    }

    #[tokio::test]
    #[cfg(feature = "pkgs-async")]
    async fn test_search_async() {
        println!("std_test_search_async");
        let std = Std::new().unwrap();
        let response = std.search("linux").await.unwrap();

        assert_eq!(
            response.results.first().unwrap().pkg_name,
            "aarch64-linux-gnu-binutils"
        );
    }

    #[test]
    #[cfg(not(feature = "pkgs-async"))]
    fn test_info() {
        println!("std_test_info");
        let std = Std::new().unwrap();
        let response = std.info("linux", Repo::Core).unwrap();

        assert_eq!(response.pkg_name, "linux");
    }

    #[tokio::test]
    #[cfg(feature = "pkgs-async")]
    async fn test_info_async() {
        println!("std_test_info_async");
        let std = Std::new().unwrap();
        let response = std.info("linux", Repo::Core).await.unwrap();

        assert_eq!(response.pkg_name, "linux");
    }
}
