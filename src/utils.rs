use std::fmt::Result;

use url::{ParseError, Url};

pub fn validate_url(raw: &str) -> std::result::Result<Url, ParseError> {
    raw.parse::<Url>()
}
