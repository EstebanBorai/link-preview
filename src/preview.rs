#[cfg(feature = "fetch")]
use reqwest::get;

use scraper::Html;

use crate::html::{find_link, find_meta_tag, first_inner_html};
use crate::og::{find_og_tag, OpenGraphTag};
use crate::twitter::{find_twitter_tag, TwitterMetaTag};

#[derive(Debug)]
pub struct LinkPreview {
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
}

impl LinkPreview {
    /// Fetches the provided URL and retrieves an instance of `LinkPreview`
    #[cfg(feature = "fetch")]
    pub async fn fetch(url: &str) -> Self {
        let resp = get(url).await.unwrap();
        let html = resp.text().await;

        LinkPreview::from_html(html)
    }

    /// Parses the provided HTML string slice and retrieves an instance of
    /// `LinkPreview`
    pub fn scan(html: &str) -> Self {
        let html = Html::parse_document(html);

        LinkPreview {
            title: LinkPreview::find_first_title(&html),
            description: LinkPreview::find_first_description(&html),
            domain: LinkPreview::find_first_domain(&html),
            image_url: LinkPreview::find_first_image_url(&html),
        }
    }

    /// Attempts to find the description of the page in the following order:
    ///
    /// - Document's `<link rel="canonical" /> element's `href` attribute
    /// - OpenGraphTag's image meta tag (`og:image`)
    pub fn find_first_domain(html: &Html) -> Option<String> {
        if let Some(domain) = find_link(html, "canonical") {
            return Some(domain);
        }

        if let Some(domain) = find_og_tag(html, OpenGraphTag::Url) {
            return Some(domain);
        }

        None
    }

    /// Attempts to find the description of the page in the following order:
    ///
    /// - OpenGraphTag's image meta tag (`og:image`)
    /// - Document's `<link rel="image_url" /> element's `href` attribute
    /// - Twitter Card's image meta tag (`twitter:image`)
    pub fn find_first_image_url(html: &Html) -> Option<String> {
        if let Some(image_url) = find_og_tag(html, OpenGraphTag::Image) {
            return Some(image_url);
        }

        if let Some(image_url) = find_link(html, "image_src") {
            return Some(image_url);
        }

        if let Some(image_url) = find_twitter_tag(html, TwitterMetaTag::Image) {
            return Some(image_url);
        }

        None
    }

    /// Attempts to find the description of the page in the following order:
    ///
    /// - OpenGraphTag's description meta tag (`og:description`)
    /// - Twitter Card's description meta tag (`twitter:description`)
    /// - Description meta tag (`description`)
    /// - The first `p` element from the document
    pub fn find_first_description(html: &Html) -> Option<String> {
        if let Some(description) = find_og_tag(html, OpenGraphTag::Description) {
            return Some(description);
        }

        if let Some(description) = find_twitter_tag(html, TwitterMetaTag::Description) {
            return Some(description);
        }

        if let Some(description) = find_meta_tag(html, "description") {
            return Some(description);
        }

        if let Some(description) = first_inner_html(html, "p") {
            return Some(description);
        }

        None
    }

    /// Attempts to find the title of the page in the following order:
    ///
    /// - OpenGraphTag's title meta tag (`og:title`)
    /// - Twitter Card's title meta tag (`twitter:title`)
    /// - The HTML's document title
    /// - The first `<h1>` tag in the document
    /// - The first `<h2>` tag in the document
    pub fn find_first_title(html: &Html) -> Option<String> {
        if let Some(title) = find_og_tag(html, OpenGraphTag::Title) {
            return Some(title);
        }

        if let Some(title) = find_twitter_tag(html, TwitterMetaTag::Title) {
            return Some(title);
        }

        if let Some(title) = first_inner_html(html, "title") {
            return Some(title);
        }

        if let Some(title) = first_inner_html(html, "h1") {
            return Some(title);
        }

        if let Some(title) = first_inner_html(html, "h2") {
            return Some(title);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::html_from_bytes;

    use super::LinkPreview;

    // #[test]
    // fn finds_first_title() {
    //     let html = html_from_bytes(APPLE_COM).unwrap();
    //     let title = LinkPreview::find_first_title(&html);

    //     assert_eq!(title.unwrap(), "Apple");
    // }

    // #[test]
    // fn finds_first_description() {
    //     let html = html_from_bytes(APPLE_COM).unwrap();
    //     let description = LinkPreview::find_first_description(&html);

    //     assert_eq!(description.unwrap(), "Discover the innovative world of Apple and shop everything iPhone, iPad, Apple Watch, Mac, and Apple TV, plus explore accessories, entertainment, and expert device support.");
    // }

    // #[test]
    // fn finds_first_image_url() {
    //     let html = html_from_bytes(APPLE_COM).unwrap();
    //     let image_url = LinkPreview::find_first_image_url(&html);

    //     assert_eq!(image_url.unwrap(), "https://www.apple.com/ac/structured-data/images/open_graph_logo.png?201809210816");
    // }

    // #[test]
    // fn finds_first_domain() {
    //     let html = html_from_bytes(APPLE_COM).unwrap();
    //     let domain = LinkPreview::find_first_domain(&html);

    //     assert_eq!(domain.unwrap(), "https://www.apple.com/");
    // }
}
