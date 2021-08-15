# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.3] - 2021-08-15
### Added
- Add `remove_html_tags`

### Fixed
- `LinkPreview.domain` is now a `Option<String>` instead of `Option<Url>`
- Removed `domain_str`

### Fixed
- `find_first_domain` returns a domain instead of a URL
- Use `Option<String>` for `LinkPreview.domain` instead of `Option<Url>`

## [0.0.2] - 2021-08-10
### Added
- `fetch` feature
  - `fetch` function to retrieve HTML
  - `fetch_partially` to fetch first 10 chunks (of arbitrary size) from the URL
  - `fetch_with_limit` same implementation for `fetch_partially` with custom
    limit of chunks

## [0.0.1] - 2021-07-31
### Added
- `LinkPreview` struct implementation
- Introduces `fetch` feature to fetch sites using `reqwest`
- Basic support for Open Graph tags scrapping
- Basic support for Twitter Cards tags scrapping
- Basic support for Shema.org tags scrapping
