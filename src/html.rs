use regex::Regex;
use scraper::{Html, Selector};

pub fn first_inner_html(html: &Html, tag: &str) -> Option<String> {
    let selector = Selector::parse(tag).unwrap();

    if let Some(element) = html.select(&selector).next() {
        let value = element.inner_html();

        if !value.is_empty() {
            return Some(value);
        }
    }

    None
}

pub fn find_meta_tag(html: &Html, property: &str) -> Option<String> {
    let selector = Selector::parse(&format!("meta[property=\"{property}\"]")).unwrap();

    if let Some(element) = html.select(&selector).next() {
        if let Some(value) = element.value().attr("content") {
            return Some(value.to_string());
        }
    }

    None
}

pub fn find_link(html: &Html, rel: &str) -> Option<String> {
    let selector = Selector::parse(&format!("link[rel=\"{rel}\"]")).unwrap();

    if let Some(element) = html.select(&selector).next() {
        if let Some(value) = element.value().attr("href") {
            return Some(value.to_string());
        }
    }

    None
}

/// Removes HTML tags from the provided HTML text
pub fn remove_html_tags(text: &str) -> String {
    let re = Regex::new("<(.|\n)*?>").unwrap();
    let res = re.replace_all(text, "");

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::remove_html_tags;

    #[test]
    fn sanitizes_html_text() {
        let html = "<html><body><p>Hello <b>World</b>!.<br /> This is our<sup>1st</sup> test on sanitization for HTML text</p><body></html>";
        let sanitized = remove_html_tags(html);

        assert_eq!(
            sanitized,
            "Hello World!. This is our1st test on sanitization for HTML text"
        );
    }
}
