use super::{Icon,IconScraper};
use std::str::FromStr;


pub trait Strategy {
    fn get_guesses(self, &mut IconScraper) -> Vec<Icon>;
}

pub struct DefaultFaviconPathStrategy;
impl Strategy for DefaultFaviconPathStrategy {
    fn get_guesses(self, parser: &mut IconScraper) -> Vec<Icon> {
        let mut icon = Icon::from_url(parser.document_url.join("/favicon.ico").unwrap());
        if icon.fetch().is_ok() {
            vec![icon]
        } else {
            vec![]
        }
    }
}

pub struct LinkRelStrategy;
impl Strategy for LinkRelStrategy {
    fn get_guesses(self, parser: &mut IconScraper) -> Vec<Icon> {
        let mut rv = vec![];
        let dom = match parser.dom {
            Some(ref x) => x,
            None => return rv
        };

        for data in dom.select("link[rel*=icon]").unwrap() {
            let attrs = data.attributes.borrow();
            let href = match attrs.get(atom!("href")) {
                Some(x) => x,
                None => continue
            };

            let icon_url = match parser.document_url.join(href) {
                Ok(x) => x,
                Err(_) => continue
            };

            let mut sizes = attrs.get(atom!("sizes"))
                .unwrap_or("")
                .split('x')
                .filter_map(|d| u32::from_str(d).ok());

            let (x, y) = match (sizes.next(), sizes.next()) {
                (Some(x), Some(y)) => (Some(x), Some(y)),
                _ => (None, None)
            };

            rv.push({
                let mut icon = Icon::from_url(icon_url);
                icon.width = x;
                icon.height = y;
                icon
            });
        };

        rv
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::IconScraper;

    use url;
    use kuchiki;
    use kuchiki::traits::*;

    #[test]
    fn test_apple_touch_icon_without_size_attr() {
        // laverna.cc does this.
        let mut scraper = IconScraper {
            document_url: url::Url::parse("http://example.com/").unwrap(),
            dom: Some(kuchiki::parse_html().one("<!DOCTYPE html>
            <html>
                <head>
                    <link rel=apple-touch-icon href=apple-touch-icon.png>
                </head>
                <body></body>
            </html>
            "))
        };

        let mut icons = LinkRelStrategy.get_guesses(&mut scraper);
        assert_eq!(icons.len(), 1);
        assert_eq!(icons.pop().unwrap().url, url::Url::parse("http://example.com/apple-touch-icon.png").unwrap());
    }

    #[test]
    fn test_sharesome() {
        let mut scraper = IconScraper::from_http("https://sharesome.5apps.com/");
        assert_eq!(scraper.fetch_icons().largest().unwrap().url, url::Url::parse("https://sharesome.5apps.com/application_icon_x512.png").unwrap());
    }
}
