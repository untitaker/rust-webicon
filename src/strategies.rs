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

        for data in dom.select("link[rel=icon], link[rel=apple-touch-icon]").unwrap() {
            let attrs = data.attributes.borrow();
            let href = match attrs.get(atom!("href")) {
                Some(x) => x,
                None => continue
            };
            let icon_url = match parser.document_url.join(href) {
                Ok(x) => x,
                Err(_) => continue
            };

            let mut sizes = match attrs.get(atom!("sizes")) {
                Some(s) => s.split('x').filter_map(|d| u32::from_str(d).ok()),
                None => continue
            };
                
            let (x, y) = match (sizes.next(), sizes.next()) {
                (Some(x), Some(y)) => (Some(x), Some(y)),
                _ => continue
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
