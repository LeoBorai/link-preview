use scraper::{Html, Selector};

/// Schema.org meta tags.
pub enum SchemaMetaTag {
    Name,
    Description,
    Image,
}

impl SchemaMetaTag {
    fn str(&self) -> &str {
        match self {
            SchemaMetaTag::Name => "name",
            SchemaMetaTag::Description => "description",
            SchemaMetaTag::Image => "image",
        }
    }
}

/// Finds the Schema.org tag specified in the provided `Html` instance
pub fn find_schema_tag(html: &Html, tag: SchemaMetaTag) -> Option<String> {
    let selector = Selector::parse(&format!("meta[itemprop=\"{}\"]", tag.str())).unwrap();

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
    use crate::tests::SCHEMA_COMPLIANT_HTML;

    use super::{find_schema_tag, SchemaMetaTag};

    #[test]
    fn retrieves_schema_name() {
        let html = html_from_bytes(SCHEMA_COMPLIANT_HTML).unwrap();
        let value = find_schema_tag(&html, SchemaMetaTag::Name).unwrap();

        assert_eq!(value, "Schema.org tags are awesome");
    }

    #[test]
    fn retrieves_schema_description() {
        let html = html_from_bytes(SCHEMA_COMPLIANT_HTML).unwrap();
        let value = find_schema_tag(&html, SchemaMetaTag::Description).unwrap();

        assert_eq!(
            value,
            "Tips to set Schema.org tags like you have been doing it since year 1"
        );
    }

    #[test]
    fn retrieves_schema_image() {
        let html = html_from_bytes(SCHEMA_COMPLIANT_HTML).unwrap();
        let value = find_schema_tag(&html, SchemaMetaTag::Image).unwrap();

        assert_eq!(value, "https://www.example.com/image.jpg");
    }
}
