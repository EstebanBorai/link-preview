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
