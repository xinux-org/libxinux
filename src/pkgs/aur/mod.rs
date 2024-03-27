pub mod schema;

use url::Url;

use crate::error::{Error, Result};
pub use schema::*;

const BASE_URL: &str = "https://aur.archlinux.org/rpc/?v=5";

pub struct Aur {
    url: Url,
}

impl Aur {
    pub fn new(base_url: Option<String>) -> Result<Aur> {
        let url = match base_url {
            Some(url) => Url::parse(&url),
            None => Url::parse(BASE_URL),
        };

        let url = match url {
            Ok(url) => url,
            Err(e) => return Err(e.into()),
        };

        Ok(Aur { url })
    }

    #[cfg(not(feature = "async"))]
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

        Ok(response)
    }

    #[cfg(feature = "async")]
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

        Ok(response)
    }

    #[cfg(not(feature = "async"))]
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

        if response.result_count == 0 {
            return Err(Error::NoResults(name.to_string()));
        }

        Ok(response.results.first().unwrap().clone())
    }

    #[cfg(feature = "async")]
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
        println!("test_new");
        let aur = Aur::new(None).unwrap();
        assert_eq!(aur.url.as_str(), BASE_URL);
    }

    #[test]
    #[cfg(not(feature = "async"))]
    fn test_search() {
        println!("test_search");
        let aur = Aur::new(None).unwrap();
        let response = aur.search("archlinux-hello", None).unwrap();

        assert_eq!(response.results.first().unwrap().id, 1193389);
    }

    #[tokio::test]
    #[cfg(feature = "async")]
    async fn test_search_async() {
        println!("test_search_async");
        let aur = Aur::new(None).unwrap();
        let response = aur.search("archlinux-hello", None).await.unwrap();

        assert_eq!(response.results.first().unwrap().id, 1193389);
    }

    #[test]
    #[cfg(not(feature = "async"))]
    fn test_info() {
        println!("test_info");
        let aur = Aur::new(None).unwrap();
        let response = aur.info("archlinux-hello").unwrap();

        assert_eq!(response.id, 1193389);
    }

    #[tokio::test]
    #[cfg(feature = "async")]
    async fn test_info_async() {
        println!("test_info_async");
        let aur = Aur::new(None).unwrap();
        let response = aur.info("archlinux-hello").await.unwrap();

        assert_eq!(response.id, 1193389);
    }
}
