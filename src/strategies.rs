use super::{Icon,IconScraper};
use std::str::FromStr;

pub trait Strategy {
    fn get_guesses(self, &mut IconScraper) -> Vec<Icon>;
}

pub struct DefaultFaviconPathStrategy;
impl DefaultFaviconPathStrategy {
    fn get_uneducated_guesses(self, parser: &mut IconScraper) -> Vec<Icon> {
        let mut current = parser.document_url.clone();
        let mut rv = vec![];

        loop {
            {
                let mut path = current.path_mut().unwrap();
                path.pop();
                path.push("favicon.ico".to_owned());
            }

            let icon = Icon::from_url(current.clone());
            rv.push(icon);

            {
                let mut path = current.path_mut().unwrap();
                path.pop().unwrap();
                if path.len() == 0 {
                    break;
                }
            }
        }
        rv
    }
}
impl Strategy for DefaultFaviconPathStrategy {
    fn get_guesses(self, parser: &mut IconScraper) -> Vec<Icon> {
        for mut icon in self.get_uneducated_guesses(parser).into_iter() {
            if icon.fetch_dimensions().is_ok() {
                return vec![icon];
            }
        }
        vec![]
    }
}

pub struct LinkRelStrategy;
impl Strategy for LinkRelStrategy {
    fn get_guesses(self, parser: &mut IconScraper) -> Vec<Icon> {
        parser.dom.as_ref().unwrap().select("link[rel=icon], link[rel=apple-touch-icon]").unwrap()
            .filter_map(|data| {
                let attrs = data.attributes.borrow();
                let href = match attrs.get(atom!("href")) {
                    Some(x) => x,
                    None => return None
                };
                let icon_url = match parser.document_url.join(href) {
                    Ok(x) => x,
                    Err(_) => return None
                };

                let mut sizes = match attrs.get(atom!("sizes")) {
                    Some(s) => s.split('x').filter_map(|d| u32::from_str(d).ok()),
                    None => return None
                };
                    
                let (x, y) = match (sizes.next(), sizes.next()) {
                    (Some(x), Some(y)) => (Some(x), Some(y)),
                    _ => return None
                };

                Some({
                    let mut icon = Icon::from_url(icon_url);
                    icon.width = x;
                    icon.height = y;
                    icon
                })
            })
            .collect()
    }
}

#[test]
fn test_default_favicon_paths() {
    use url;
    let mut x = IconScraper::from_url(
        url::Url::parse("http://example.com/a/b/c/d/e/f").unwrap()
    );
    let s = DefaultFaviconPathStrategy;
    let paths = s.get_uneducated_guesses(&mut x)
        .into_iter()
        .map(|u| u.url.serialize_path().unwrap())
        .collect::<Vec<String>>();

    assert_eq!(paths, vec![
        "/a/b/c/d/e/favicon.ico",
        "/a/b/c/d/favicon.ico",
        "/a/b/c/favicon.ico",
        "/a/b/favicon.ico",
        "/a/favicon.ico",
        "/favicon.ico",
    ]);
}
