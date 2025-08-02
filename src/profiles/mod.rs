use scraper::Html;
use url::Url;

use crate::LinkPreview;

pub mod youtube;

pub trait ProfileExt: Send + Sync + Sized {
    /// Checks if the profile fits the given URL.
    fn fits(url: &Url) -> bool;

    /// Creates a `LinkPreview` from the provided HTML.
    fn extract(html: &Html) -> Option<LinkPreview>;
}
