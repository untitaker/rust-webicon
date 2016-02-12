# rust-webicon

[![Build Status](https://travis-ci.org/untitaker/rust-webicon.svg?branch=master)](https://travis-ci.org/untitaker/rust-webicon)

* [Repository](https://github.com/untitaker/rust-webicon)
* [Documentation](https://rust-webicon.unterwaditzer.net/webicon/)

A library for scraping a website's icon.

Usage:

    extern crate webicon;
    use webicon::IconScraper;

    let mut scraper = IconScraper::from_http("http://twitter.com").unwrap();
    let icons = scraper.fetch_icons();

    icons.at_least(128, 128);  // Return icon that is at least 128x128 pixels large
    icons.largest();  // Just return the largest one.

Read more in the [docs](https://rust-webicon.unterwaditzer.net/).

## License

Licensed under the MIT, see `LICENSE`.
