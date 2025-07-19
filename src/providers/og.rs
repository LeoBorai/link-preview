//! Open Graph Protocol bindings
//!
//! The Open Graph protocol enables any web page to become a rich object in a
//! social graph. For instance, this is used on Facebook to allow any web page
//! to have the same functionality as any other object on Facebook.
//!
//!
//!
//! # References
//! - [Official Documentation](https://ogp.me)
use scraper::{Html, Selector};
use std::fmt;

/// OpenGraphTag meta tags collection
pub enum OpenGraphTag {
    /// Represents the "og:title" OpenGraph meta tag.
    ///
    /// The title of your object as it should appear within
    /// the graph, e.g., "The Rock".
    Title,
    /// Represents the "og:url" OpenGraph meta tag
    Url,
    /// Represents the "og:image" OpenGraph meta tag
    Image,
    /// Represents the "og:type" OpenGraph meta tag
    ///
    /// The type of your object, e.g., "video.movie". Depending on the type
    /// you specify, other properties may also be required.
    Type,
    /// Represents the "og:description" OpenGraph meta tag
    Description,
    /// Represents the "og:locale" OpenGraph meta tag
    Locale,
    /// Represents the "og:image:height" OpenGraph meta tag
    ImageHeight,
    /// Represents the "og:image:width" OpenGraph meta tag
    ImageWidth,
    /// Represents the "og:site_name" OpenGraph meta tag
    SiteName,
}

impl fmt::Debug for OpenGraphTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.str())
    }
}

impl OpenGraphTag {
    fn str(&self) -> &str {
        match self {
            OpenGraphTag::Title => "title",
            OpenGraphTag::Url => "url",
            OpenGraphTag::Image => "image",
            OpenGraphTag::Type => "type",
            OpenGraphTag::Description => "description",
            OpenGraphTag::Locale => "locale",
            OpenGraphTag::ImageHeight => "image:height",
            OpenGraphTag::ImageWidth => "image:width",
            OpenGraphTag::SiteName => "site_name",
        }
    }
}

/// Finds the OpenGraphTag tag specified in the provided `Html` instance
pub fn find_og_tag(html: &Html, tag: OpenGraphTag) -> Option<String> {
    let selector = Selector::parse(&format!("meta[property=\"og:{}\"]", tag.str())).unwrap();

    if let Some(element) = html.select(&selector).next() {
        if let Some(value) = element.value().attr("content") {
            return Some(value.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::html_from_bytes;
    use crate::tests::OG_COMPLIANT_HTML;

    use super::{find_og_tag, OpenGraphTag};

    #[test]
    fn retrieves_title_from_og_compliant_html() {
        let html = html_from_bytes(OG_COMPLIANT_HTML).unwrap();
        let title = find_og_tag(&html, OpenGraphTag::Title).unwrap();

        assert_eq!(title, "SEO Strategies for a better web");
    }

    #[test]
    fn retrieves_description_from_og_compliant_html() {
        let html = html_from_bytes(OG_COMPLIANT_HTML).unwrap();
        let description = find_og_tag(&html, OpenGraphTag::Description).unwrap();

        assert_eq!(description, "John Appleseed tells you his secrets on SEO for a better web experience by taking advantage of OpenGraph\'s Tags!");
    }

    #[test]
    fn retrieves_url_from_og_compliant_html() {
        let html = html_from_bytes(OG_COMPLIANT_HTML).unwrap();
        let url = find_og_tag(&html, OpenGraphTag::Url).unwrap();

        assert_eq!(url, "https://abetterweb.com");
    }

    #[test]
    fn retrieves_locale_from_og_compliant_html() {
        let html = html_from_bytes(OG_COMPLIANT_HTML).unwrap();
        let locale = find_og_tag(&html, OpenGraphTag::Locale).unwrap();

        assert_eq!(locale, "en_US");
    }

    #[test]
    fn retrieves_image_from_og_compliant_html() {
        let html = html_from_bytes(OG_COMPLIANT_HTML).unwrap();
        let image = find_og_tag(&html, OpenGraphTag::Image).unwrap();

        assert_eq!(
            image,
            "https://www.apple.com/ac/structured-data/images/open_graph_logo.png?201809210816"
        );
    }

    #[test]
    fn retrieves_type_from_og_compliant_html() {
        let html = html_from_bytes(OG_COMPLIANT_HTML).unwrap();
        let r#type = find_og_tag(&html, OpenGraphTag::Type).unwrap();

        assert_eq!(r#type, "website");
    }

    #[test]
    fn retrieves_image_height_from_og_compliant_html() {
        let html = html_from_bytes(OG_COMPLIANT_HTML).unwrap();
        let height = find_og_tag(&html, OpenGraphTag::ImageHeight).unwrap();

        assert_eq!(height, "600");
    }

    #[test]
    fn retrieves_image_width_from_og_compliant_html() {
        let html = html_from_bytes(OG_COMPLIANT_HTML).unwrap();
        let width = find_og_tag(&html, OpenGraphTag::ImageWidth).unwrap();

        assert_eq!(width, "1200");
    }

    #[test]
    fn retrieves_site_name_from_og_compliant_html() {
        let html = html_from_bytes(OG_COMPLIANT_HTML).unwrap();
        let site_name = find_og_tag(&html, OpenGraphTag::SiteName).unwrap();

        assert_eq!(site_name, "TechPro");
    }
}
