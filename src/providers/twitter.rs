use scraper::{Html, Selector};

/// Twittet meta tags.
pub enum TwitterMetaTag {
    /// Title for the Twitter card
    Title,
    /// The card type, which will be one of “summary”, “summary_large_image”,
    /// “app”, or “player”.
    Card,
    /// @username for the website used in the card footer.
    /// This tag is not required, make sure the `Option::None` variant is handled.
    Site,
    /// @username for the content creator / author.
    /// This tag is not required, make sure the `Option::None` variant is handled.
    Creator,
    /// Twitter Card Image
    Image,
    /// Card description
    Description,
}

impl TwitterMetaTag {
    fn str(&self) -> &str {
        match self {
            TwitterMetaTag::Title => "title",
            TwitterMetaTag::Card => "card",
            TwitterMetaTag::Site => "site",
            TwitterMetaTag::Creator => "creator",
            TwitterMetaTag::Image => "image",
            TwitterMetaTag::Description => "description",
        }
    }
}

/// Finds the Twitter tag specified in the provided `Html` instance
pub fn find_twitter_tag(html: &Html, tag: TwitterMetaTag) -> Option<String> {
    let selector = Selector::parse(&format!("meta[name=\"twitter:{}\"]", tag.str())).unwrap();

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
    use crate::tests::TWITTER_COMPLIANT_HTML;

    use super::{find_twitter_tag, TwitterMetaTag};

    #[test]
    fn retrieves_card() {
        let html = html_from_bytes(TWITTER_COMPLIANT_HTML).unwrap();
        let value = find_twitter_tag(&html, TwitterMetaTag::Card).unwrap();

        assert_eq!(value, "summary_large_image");
    }

    #[test]
    fn retrieves_title() {
        let html = html_from_bytes(TWITTER_COMPLIANT_HTML).unwrap();
        let value = find_twitter_tag(&html, TwitterMetaTag::Title).unwrap();

        assert_eq!(value, "SEO Strategies for a better web");
    }

    #[test]
    fn retrieves_image() {
        let html = html_from_bytes(TWITTER_COMPLIANT_HTML).unwrap();
        let value = find_twitter_tag(&html, TwitterMetaTag::Image).unwrap();

        assert_eq!(value, "https://linktoyourimage");
    }

    #[test]
    fn retrieves_description() {
        let html = html_from_bytes(TWITTER_COMPLIANT_HTML).unwrap();
        let value = find_twitter_tag(&html, TwitterMetaTag::Description).unwrap();

        assert_eq!(value, "John Appleseed tells you his secrets on SEO for a better web experience by taking advantage of OpenGraph's Tags!");
    }

    #[test]
    fn retrieves_tweet_site_name() {
        let html = html_from_bytes(TWITTER_COMPLIANT_HTML).unwrap();
        let value = find_twitter_tag(&html, TwitterMetaTag::Site).unwrap();

        assert_eq!(value, "@nytimes");
    }

    #[test]
    fn retrieves_tweet_creator_username() {
        let html = html_from_bytes(TWITTER_COMPLIANT_HTML).unwrap();
        let value = find_twitter_tag(&html, TwitterMetaTag::Creator).unwrap();

        assert_eq!(value, "@EstebanBorai");
    }
}
