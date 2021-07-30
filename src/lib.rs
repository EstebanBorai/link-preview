pub mod html;
pub mod og;
pub mod preview;
pub mod schema;
pub mod twitter;
pub mod utils;

pub use preview::LinkPreview;

use scraper::Html;
use std::string::FromUtf8Error;

pub fn html_from_bytes(value: &[u8]) -> Result<Html, FromUtf8Error> {
    let utf8 = String::from_utf8(value.to_vec())?;

    Ok(Html::parse_document(utf8.as_str()))
}

#[cfg(test)]
mod tests {
    pub const COMMON_HTML: &[u8] = include_bytes!("../html/common.html");
    pub const FULL_FEATURED_HTML: &[u8] = include_bytes!("../html/full_featured.html");
    pub const OG_COMPILANT_HTML: &[u8] = include_bytes!("../html/og_compilant.html");
    pub const TWITTER_COMPILANT_HTML: &[u8] = include_bytes!("../html/twitter_compilant.html");
}
