pub mod html;
pub mod preview;
pub mod providers;

pub use preview::{html_from_bytes, LinkPreview};

#[cfg(feature = "fetch")]
pub mod fetch;

#[cfg(test)]
mod tests {
    pub const FULL_FEATURED_HTML: &[u8] = include_bytes!("../html/full_featured.html");
    pub const OG_COMPLIANT_HTML: &[u8] = include_bytes!("../html/og_compliant.html");
    pub const SCHEMA_COMPLIANT_HTML: &[u8] = include_bytes!("../html/schema_compliant.html");
    pub const TWITTER_COMPLIANT_HTML: &[u8] = include_bytes!("../html/twitter_compliant.html");

    #[cfg(feature = "fetch")]
    pub const REMOTE_FULL_FEATURED_HTML: &str =
        "https://raw.githubusercontent.com/EstebanBorai/link-preview/main/html/full_featured.html";
}
