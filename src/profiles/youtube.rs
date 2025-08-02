use scraper::Html;
use url::Url;

use crate::profiles::ProfileExt;
use crate::LinkPreview;

const YOUTUBE_IMAGE_STORAGE_DOMAIN: &str = "https://i.ytimg.com";

pub struct YouTubeProfile {}

impl ProfileExt for YouTubeProfile {
    fn extract(html: &Html) -> Option<LinkPreview> {
        let mut link_preview = LinkPreview::from(html);

        if let Some(image_url) = link_preview.image_url {
            let mut url = Url::parse(YOUTUBE_IMAGE_STORAGE_DOMAIN).ok()?;
            url.set_path(image_url.path());
            link_preview.image_url = Some(url);
        }

        Some(link_preview)
    }

    fn fits(url: &Url) -> bool {
        url.host_str()
            .is_some_and(|host| host.contains("youtube.com") || host.contains("youtu.be"))
    }
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use scraper::Html;

    use crate::tests::YOUTUBE_VIDEO_HTML;

    use super::*;

    #[test]
    fn test_youtube_profile() {
        let html = from_utf8(YOUTUBE_VIDEO_HTML).expect("Failed to parse YouTube HTML");
        let html = Html::parse_document(html);

        let url = Url::parse("https://youtu.be/61JHONRXhjs").expect("Failed to parse URL");
        assert!(YouTubeProfile::fits(&url));

        let link_preview = YouTubeProfile::extract(&html);
        assert!(link_preview.is_some());

        let preview = link_preview.unwrap();

        assert_eq!(
            preview.title,
            Some("Google — Year in Search 2024".to_string())
        );

        assert_eq!(
            preview.description,
            Some("This year, we're celebrating the Breakout Searches of 2024. From iconic performances, to history-making breakthroughs, see the moments that shaped our year i...".to_string())
        );

        assert_eq!(
            preview.image_url.map(|u| u.to_string()),
            Some("https://i.ytimg.com/vi/61JHONRXhjs/maxresdefault.jpg".to_string())
        );

        assert_eq!(preview.domain, Some("www.youtube.com".to_string()));
    }
}
