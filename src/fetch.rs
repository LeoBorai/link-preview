use std::string::FromUtf8Error;

use reqwest::get;
use scraper::Html;
use thiserror::Error;

#[cfg(feature = "blocking")]
use reqwest::blocking::get as blocking_get;

#[cfg(feature = "stream")]
use futures_util::TryStreamExt;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to fetch {0}. An error ocurred: {1}")]
    FetchFailed(String, reqwest::Error),
    #[error("Failed to parse response from {0}. An error ocurred: {1}")]
    ParseError(String, reqwest::Error),
    #[error("Failed to stream response chunks {0}. An error ocurred: {1}")]
    StreamError(String, reqwest::Error),
    #[error("Failed to parse bytes into UTF-8 while streaming response from {0}")]
    InvalidUtf8(String, FromUtf8Error),
}

/// Fetches the provided URL and retrieves an instance of `Html`
pub async fn fetch(url: &str) -> Result<Html, Error> {
    let resp = get(url)
        .await
        .map_err(|err| Error::FetchFailed(url.to_string(), err))?;
    let html = resp
        .text()
        .await
        .map_err(|err| Error::ParseError(url.to_string(), err))?;

    Ok(Html::parse_document(&html))
}

#[cfg(feature = "blocking")]
pub fn fetch_blocking(url: &str) -> Result<Html, Error> {
    let resp = blocking_get(url).map_err(|err| Error::FetchFailed(url.to_string(), err))?;
    let html = resp
        .text()
        .map_err(|err| Error::ParseError(url.to_string(), err))?;

    Ok(Html::parse_document(&html))
}

/// Fetches the provided URL and retrieves an instance of `LinkPreview`
#[cfg(feature = "stream")]
pub async fn fetch_partially(url: &str) -> Result<Html, Error> {
    fetch_with_limit(url, 10).await
}

/// Fetches the provided URL and retrieves an instance of `LinkPreview`
#[cfg(feature = "stream")]
pub async fn fetch_with_limit(url: &str, limit: usize) -> Result<Html, Error> {
    let mut laps = 0_usize;
    let mut resp = get(url)
        .await
        .map_err(|err| Error::FetchFailed(url.to_string(), err))?
        .bytes_stream();
    let mut bytes: Vec<u8> = Vec::new();

    while let Some(chunk) = resp
        .try_next()
        .await
        .map_err(|err| Error::StreamError(url.to_string(), err))?
    {
        if laps >= limit {
            break;
        }

        let ref mut chunk = chunk.to_vec();

        bytes.append(chunk);
        laps += 1;
    }

    let html = String::from_utf8(bytes).map_err(|err| Error::InvalidUtf8(url.to_string(), err))?;

    Ok(Html::parse_document(&html))
}

#[cfg(test)]
mod tests {
    use crate::tests::REMOTE_FULL_FEATURED_HTML;
    use crate::LinkPreview;

    use super::{fetch, fetch_partially, fetch_with_limit};

    #[tokio::test]
    async fn fetches() {
        let html = fetch(REMOTE_FULL_FEATURED_HTML).await.unwrap();
        let link_preview = LinkPreview::from(&html);

        assert_eq!(
            link_preview.title.unwrap_or(String::default()),
            "SEO Strategies for a better web"
        );
        assert_eq!(link_preview.description.unwrap_or(String::default()), "John Appleseed tells you his secrets on SEO for a better web experience by taking advantage of OpenGraph\'s Tags!");
    }

    #[tokio::test]
    async fn fetches_page_partially() {
        let html = fetch_partially(REMOTE_FULL_FEATURED_HTML).await.unwrap();
        let link_preview = LinkPreview::from(&html);

        assert_eq!(
            link_preview.title.unwrap_or(String::default()),
            "SEO Strategies for a better web"
        );
        assert_eq!(link_preview.description.unwrap_or(String::default()), "John Appleseed tells you his secrets on SEO for a better web experience by taking advantage of OpenGraph\'s Tags!");
    }

    #[tokio::test]
    async fn fetches_page_with_limit_of_20() {
        let html = fetch_with_limit(REMOTE_FULL_FEATURED_HTML, 20)
            .await
            .unwrap();
        let link_preview = LinkPreview::from(&html);

        assert_eq!(
            link_preview.title.unwrap_or(String::default()),
            "SEO Strategies for a better web"
        );
        assert_eq!(link_preview.description.unwrap_or(String::default()), "John Appleseed tells you his secrets on SEO for a better web experience by taking advantage of OpenGraph\'s Tags!");
    }
}
