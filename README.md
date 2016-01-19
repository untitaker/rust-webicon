# rust-webicon

A library for scraping a website's icon.

Usage:

    extern crate webicon;
    use webicon::IconScraper;

    let mut scraper = IconScraper::from_url("http://twitter.com");
    try!(scraper.fetch_document());
    scraper.fetch_icons();

    scraper.at_least(128, 128);  // Return icon that is at least 128x128 pixels large
    scraper.largest();  // Just return the largest one.
    scraper.icons;  // A Vec<Icon> that contains all variants

Read more in the [docs](https://rust-webicon.unterwaditzer.net/).

## License

Licensed under the MIT, see `LICENSE`.
