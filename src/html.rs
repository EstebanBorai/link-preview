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
    let selector = Selector::parse(&format!("meta[property=\"{}\"]", property)).unwrap();

    if let Some(element) = html.select(&selector).next() {
        if let Some(value) = element.value().attr("content") {
            return Some(value.to_string());
        }
    }

    None
}

pub fn find_link(html: &Html, rel: &str) -> Option<String> {
    let selector = Selector::parse(&format!("link[rel=\"{}\"]", rel)).unwrap();

    if let Some(element) = html.select(&selector).next() {
        if let Some(value) = element.value().attr("href") {
            return Some(value.to_string());
        }
    }

    None
}
