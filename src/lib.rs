#[cfg(feature = "fetch")]
pub mod fetch;

pub mod html;
pub mod og;
pub mod preview;
pub mod schema;
pub mod twitter;

pub use preview::{html_from_bytes, LinkPreview};

#[cfg(test)]
mod tests {
    pub const FULL_FEATURED_HTML: &[u8] = include_bytes!("../html/full_featured.html");
    pub const OG_COMPILANT_HTML: &[u8] = include_bytes!("../html/og_compilant.html");
    pub const SCHEMA_COMPILANT_HTML: &[u8] = include_bytes!("../html/schema_compilant.html");
    pub const TWITTER_COMPILANT_HTML: &[u8] = include_bytes!("../html/twitter_compilant.html");

    #[cfg(feature = "fetch")]
    pub const REMOTE_FULL_FEATURED_HTML: &str =
        "https://raw.githubusercontent.com/EstebanBorai/link-preview/main/html/full_featured.html";
}
