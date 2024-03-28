pub mod schema;

use crate::error::{Result};
use super::{aur::Aur, std::Std};
pub use schema::*;


pub trait ToAny {
    fn to_any(&self) -> Data;
}

pub struct Any {
    pub std: Std,
    pub aur: Aur,
}

impl Any {
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

    #[cfg(not(feature = "pkgs-async"))]
    pub fn search<T>(&self, query: T) -> Result<Vec<Data>>
    where
        T: AsRef<str> + Clone,
    {
        let mut data: Vec<Data> = Vec::new();

        match self.std.search(query.clone()) {
            Ok(std) => {
                std.results.iter().for_each(|item| {
                    data.push(item.to_any());
                })
            },
            Err(e) => return Err(e),
        };

        match self.aur.search(query.clone(), None) {
            Ok(aur) => {
                aur.results.iter().for_each(|item| {
                    data.push(item.to_any());
                })
            },
            Err(e) => return Err(e),
        };

        let results = Data::fuzzy_search(data, query)
            .iter()
            .map(|item| item.0.clone())
            .collect();

        Ok(results)
    }

    #[cfg(feature = "pkgs-async")]
    pub async fn search<T>(&self, query: T) -> Result<Vec<Data>>
    where
        T: AsRef<str> + Clone,
    {
        let mut data: Vec<Data> = Vec::new();

        match self.std.search(query.clone()).await {
            Ok(std) => {
                std.results.iter().for_each(|item| {
                    data.push(item.to_any());
                })
            },
            Err(e) => return Err(e),
        };

        match self.aur.search(query.clone(), None).await {
            Ok(aur) => {
                aur.results.iter().for_each(|item| {
                    data.push(item.to_any());
                })
            },
            Err(e) => return Err(e),
        };

        let results = Data::fuzzy_search(data, query)
            .iter()
            .map(|item| item.0.clone())
            .collect();

        Ok(results)
    }

    // #[cfg(not(feature = "pkgs-async"))]
    // pub fn info(&self, name: &str, repo: Repo) -> Result<crate::pkgs::std::Data> {
    //     let url = format!(
    //         "{base}/{repo}/{arch}/{name}/json",
    //         base = self.url.as_str(),
    //         repo = repo,
    //         arch = Arch::X86_64,
    //         name = name
    //     );
    //
    //     let response = match reqwest::blocking::get(url) {
    //         Ok(response) => response.json::<crate::pkgs::std::Data>(),
    //         Err(e) => return Err(e.into()),
    //     };
    //
    //     let response = match response {
    //         Ok(response) => response,
    //         Err(_) => return Err(Error::NoResults(name.to_string())),
    //     };
    //
    //     Ok(response)
    // }

    // #[cfg(feature = "pkgs-async")]
    // pub async fn info(&self, name: &str, repo: Repo) -> Result<crate::pkgs::std::Data> {
    //     let url = format!(
    //         "{base}/{repo}/{arch}/{name}/json",
    //         base = self.url.as_str(),
    //         repo = repo,
    //         arch = Arch::X86_64,
    //         name = name
    //     );
    //
    //     let response = match reqwest::get(url).await {
    //         Ok(response) => response.json::<crate::pkgs::std::Data>().await,
    //         Err(e) => return Err(e.into()),
    //     };
    //
    //     let response = match response {
    //         Ok(response) => response,
    //         Err(_) => return Err(Error::NoResults(name.to_string())),
    //     };
    //
    //     Ok(response)
    // }
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

        assert_eq!(
            response.first().unwrap().name,
            "linux"
        );
    }

    #[tokio::test]
    #[cfg(feature = "pkgs-async")]
    async fn test_search_async() {
        println!("any_test_search_async");
        let any = Any::new().unwrap();
        let response = any.search("linux").await.unwrap();

        assert_eq!(
            response.first().unwrap().name,
            "linux"
        );
    }

    // #[test]
    // #[cfg(not(feature = "pkgs-async"))]
    // fn test_info() {
    //     println!("std_test_info");
    //     let std = Std::new().unwrap();
    //     let response = std.info("linux", Repo::Core).unwrap();
    //
    //     assert_eq!(response.pkg_name, "linux");
    // }

    // #[tokio::test]
    // #[cfg(feature = "pkgs-async")]
    // async fn test_info_async() {
    //     println!("std_test_info_async");
    //     let std = Std::new().unwrap();
    //     let response = std.info("linux", Repo::Core).await.unwrap();
    //
    //     assert_eq!(response.pkg_name, "linux");
    // }
}
