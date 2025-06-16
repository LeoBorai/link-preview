use std::str::FromStr;
use std::string::FromUtf8Error;

use scraper::Html;
use thiserror::Error;
use url::Url;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::html::{find_link, find_meta_tag, first_inner_html};
use crate::providers::og::{find_og_tag, OpenGraphTag};
use crate::providers::schema::{find_schema_tag, SchemaMetaTag};
use crate::providers::twitter::{find_twitter_tag, TwitterMetaTag};

#[derive(Error, Debug)]
pub enum Error {
    #[error("The provided byte slice contains invalid UTF-8 characters")]
    InvalidUtf8(FromUtf8Error),
}

/// Represents a link preview, which contains metadata about a web page
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LinkPreview {
    pub title: Option<String>,
    pub description: Option<String>,
    pub domain: Option<String>,
    pub image_url: Option<Url>,
}

impl LinkPreview {
    /// Retrieves the `String` representation of `image_url` `Url` instance
    pub fn image_url_str(&self) -> Option<String> {
        if let Some(image_url) = self.image_url.clone() {
            return Some(image_url.to_string());
        }

        None
    }

    /// Attempts to find the description of the page in the following order:
    ///
    /// - Document's `<link rel="canonical" /> element's `href` attribute
    /// - OpenGraphTag's image meta tag (`og:image`)
    pub fn find_first_domain(html: &Html) -> Option<String> {
        if let Some(domain) = find_link(html, "canonical") {
            return LinkPreview::domain_from_string(domain);
        }

        if let Some(domain) = find_og_tag(html, OpenGraphTag::Url) {
            return LinkPreview::domain_from_string(domain);
        }

        None
    }

    /// Attempts to parse a `Url` from a `String` and then attempts to retrieve
    /// the `domain` fragment from such `Url` instance.
    ///
    /// If either the `Url` is invalid or theres no a domain fragment available
    /// (the provided `Url` points to an IP instead of a domain), `None` is
    /// returned.
    fn domain_from_string(value: String) -> Option<String> {
        let url = Url::parse(&value).ok()?;

        url.domain().map(|domain| domain.to_string())
    }

    /// Attempts to find the description of the page in the following order:
    ///
    /// - OpenGraphTag's image meta tag (`og:image`)
    /// - Document's `<link rel="image_url" /> element's `href` attribute
    /// - Twitter Card's image meta tag (`twitter:image`)
    /// - Schema.org image meta tag (`image`)
    pub fn find_first_image_url(html: &Html) -> Option<Url> {
        if let Some(image_url) = find_og_tag(html, OpenGraphTag::Image) {
            return Url::parse(&image_url).ok();
        }

        if let Some(image_url) = find_link(html, "image_src") {
            return Url::parse(&image_url).ok();
        }

        if let Some(image_url) = find_schema_tag(html, SchemaMetaTag::Image) {
            return Url::parse(&image_url).ok();
        }

        if let Some(image_url) = find_twitter_tag(html, TwitterMetaTag::Image) {
            return Url::parse(&image_url).ok();
        }

        None
    }

    /// Attempts to find the description of the page in the following order:
    ///
    /// - OpenGraphTag's description meta tag (`og:description`)
    /// - Twitter Card's description meta tag (`twitter:description`)
    /// - Schema.org description meta tag (`description`)
    /// - Description meta tag (`description`)
    /// - The first `p` element from the document
    pub fn find_first_description(html: &Html) -> Option<String> {
        if let Some(description) = find_og_tag(html, OpenGraphTag::Description) {
            return Some(description);
        }

        if let Some(description) = find_twitter_tag(html, TwitterMetaTag::Description) {
            return Some(description);
        }

        if let Some(description) = find_schema_tag(html, SchemaMetaTag::Description) {
            return Some(description);
        }

        if let Some(description) = find_meta_tag(html, "description") {
            return Some(description);
        }

        if let Some(description) = first_inner_html(html, "p") {
            return Some(description);
        }

        None
    }

    /// Attempts to find the title of the page in the following order:
    ///
    /// - OpenGraphTag's title meta tag (`og:title`)
    /// - Twitter Card's title meta tag (`twitter:title`)
    /// - Schema.org title meta tag (`title`)
    /// - The HTML's document title
    /// - The first `<h1>` tag in the document
    /// - The first `<h2>` tag in the document
    pub fn find_first_title(html: &Html) -> Option<String> {
        if let Some(title) = find_og_tag(html, OpenGraphTag::Title) {
            return Some(title);
        }

        if let Some(title) = find_twitter_tag(html, TwitterMetaTag::Title) {
            return Some(title);
        }

        if let Some(title) = find_schema_tag(html, SchemaMetaTag::Name) {
            return Some(title);
        }

        if let Some(title) = first_inner_html(html, "title") {
            return Some(title);
        }

        if let Some(title) = first_inner_html(html, "h1") {
            return Some(title);
        }

        if let Some(title) = first_inner_html(html, "h2") {
            return Some(title);
        }

        None
    }
}

impl From<Html> for LinkPreview {
    fn from(html: Html) -> Self {
        let image_url: Option<Url> = LinkPreview::find_first_image_url(&html);
        let domain = LinkPreview::find_first_domain(&html);

        LinkPreview {
            title: LinkPreview::find_first_title(&html),
            description: LinkPreview::find_first_description(&html),
            domain,
            image_url,
        }
    }
}

impl From<&Html> for LinkPreview {
    fn from(html: &Html) -> Self {
        let image_url: Option<Url> = LinkPreview::find_first_image_url(html);
        let domain: Option<String> = LinkPreview::find_first_domain(html);

        LinkPreview {
            title: LinkPreview::find_first_title(html),
            description: LinkPreview::find_first_description(html),
            domain,
            image_url,
        }
    }
}

impl FromStr for LinkPreview {
    type Err = Error;

    fn from_str(html: &str) -> Result<Self, Self::Err> {
        let html = Html::parse_document(html);
        let image_url: Option<Url> = LinkPreview::find_first_image_url(&html);
        let domain: Option<String> = LinkPreview::find_first_domain(&html);

        Ok(LinkPreview {
            title: LinkPreview::find_first_title(&html),
            description: LinkPreview::find_first_description(&html),
            domain,
            image_url,
        })
    }
}

/// Attempts to convert a HTML document byte slice into a HTML string instance
/// and then parses the document into a `Html` instance
pub fn html_from_bytes(value: &[u8]) -> Result<Html, Error> {
    let utf8 = String::from_utf8(value.to_vec()).map_err(Error::InvalidUtf8)?;

    Ok(Html::parse_document(utf8.as_str()))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::html_from_bytes;
    use crate::tests::FULL_FEATURED_HTML;

    use super::LinkPreview;

    #[test]
    fn creates_instance_of_link_preview_from_html_instance() {
        let html = html_from_bytes(FULL_FEATURED_HTML).unwrap();
        let link_preview = LinkPreview::from(&html);

        assert_eq!(
            link_preview.title.unwrap(),
            "SEO Strategies for a better web"
        );
        assert_eq!(link_preview.description.unwrap(), "John Appleseed tells you his secrets on SEO for a better web experience by taking advantage of OpenGraph\'s Tags!");
        assert_eq!(
            link_preview.image_url.unwrap().to_string(),
            "https://www.apple.com/ac/structured-data/images/open_graph_logo.png?201809210816"
        );
        assert_eq!(link_preview.domain.unwrap().to_string(), "en.wikipedia.com");
    }

    #[test]
    fn creates_instance_of_link_preview_from_str_instance() {
        let html = String::from_utf8(FULL_FEATURED_HTML.to_vec()).unwrap();
        let link_preview = LinkPreview::from_str(&html).unwrap();

        assert_eq!(
            link_preview.title.unwrap(),
            "SEO Strategies for a better web"
        );
        assert_eq!(link_preview.description.unwrap(), "John Appleseed tells you his secrets on SEO for a better web experience by taking advantage of OpenGraph\'s Tags!");
        assert_eq!(
            link_preview.image_url.unwrap().to_string(),
            "https://www.apple.com/ac/structured-data/images/open_graph_logo.png?201809210816"
        );
        assert_eq!(link_preview.domain.unwrap().to_string(), "en.wikipedia.com");
    }

    #[test]
    fn finds_first_title() {
        let html = html_from_bytes(FULL_FEATURED_HTML).unwrap();
        let title = LinkPreview::find_first_title(&html);

        assert_eq!(title.unwrap(), "SEO Strategies for a better web");
    }

    #[test]
    fn finds_first_description() {
        let html = html_from_bytes(FULL_FEATURED_HTML).unwrap();
        let title = LinkPreview::find_first_description(&html);

        assert_eq!(title.unwrap(), "John Appleseed tells you his secrets on SEO for a better web experience by taking advantage of OpenGraph\'s Tags!");
    }

    #[test]
    fn finds_first_image_url() {
        let html = html_from_bytes(FULL_FEATURED_HTML).unwrap();
        let image_url: Option<String> =
            LinkPreview::find_first_image_url(&html).map(|url| url.to_string());

        assert_eq!(
            image_url.unwrap(),
            "https://www.apple.com/ac/structured-data/images/open_graph_logo.png?201809210816"
        );
    }

    #[test]
    fn finds_first_domain() {
        let html = html_from_bytes(FULL_FEATURED_HTML).unwrap();
        let domain = LinkPreview::find_first_domain(&html).map(|url| url.to_string());

        assert_eq!(domain.unwrap(), "en.wikipedia.com");
    }
}
