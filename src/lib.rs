// DOCS

#[macro_use] extern crate error_chain;
#[macro_use] extern crate html5ever_atoms;
extern crate mime;
extern crate reqwest;
extern crate url;
extern crate kuchiki;
extern crate image;
extern crate html5ever;

mod strategies;
mod util;
pub mod errors;

use errors::*;

use kuchiki::parse_html;

use reqwest::IntoUrl;

use image::GenericImage;
use strategies::Strategy;
use std::io::Read;
use util::AsImageFormat;

pub struct IconScraper {
    document_url: url::Url,
    dom: Option<kuchiki::NodeRef>,
}

impl IconScraper {
    pub fn from_http<I: IntoUrl>(url: I) -> Self {
        use html5ever::driver::BytesOpts;
        use html5ever::encoding::label::encoding_from_whatwg_label;
        use html5ever::tendril::TendrilSink;

        let url = url.into_url().unwrap();
        let dom = reqwest::get(url.clone())
            .ok()
            .and_then(|mut response| {
                let parser = parse_html();
                let opts = BytesOpts {
                    transport_layer_encoding: response.headers().get::<reqwest::header::ContentType>()
                        .and_then(|content_type| content_type.get_param(mime::Attr::Charset))
                        .and_then(|charset| encoding_from_whatwg_label(charset))
                };
                parser.from_bytes(opts).read_from(&mut response).ok()
            });

        IconScraper {
            document_url: url,
            dom: dom
        }
    }

    /// Search the document for icon metadata, also brute-force some favicon paths.
    ///
    /// **Note:** This operation is fairly costly, it is recommended to cache the results!
    ///
    /// # Panics
    ///
    /// If the document is not fetched yet.
    pub fn fetch_icons(&mut self) -> IconCollection {
        let icons = strategies::LinkRelStrategy.get_guesses(self)
            .into_iter()
            .chain(strategies::DefaultFaviconPathStrategy.get_guesses(self).into_iter())
            .filter_map(|mut icon| if icon.fetch_dimensions().is_ok() { Some(icon) } else { None })
            .collect::<Vec<_>>();

        IconCollection::from_raw(icons)
    }
}

pub struct IconCollection {
    icons: Vec<Icon>
}

impl IconCollection {
    fn from_raw(mut icons: Vec<Icon>) -> Self {
        icons.sort_by(|a, b| {
            (a.width.unwrap() * a.height.unwrap())
                .cmp(&(b.width.unwrap() * b.height.unwrap()))
        });
        IconCollection {
            icons: icons
        }
    }

    /// Return an icon that is at least of the given dimensions
    ///
    /// If there's only one icon available, it will return that icon. If there's no icon available,
    /// None is returned.
    pub fn at_least(mut self, width: u32, height: u32) -> Option<Icon> {
        let largest = self.icons.pop();
        self.icons
            .into_iter()
            .skip_while(|icon| icon.width.unwrap() < width || icon.height.unwrap() < height)
            .next()
            .or(largest)
    }

    /// Return the largest icon
    pub fn largest(mut self) -> Option<Icon> {
        self.icons.pop()
    }

    /// [unstable] Give up ownership of the inner datastructure: A vector of icons, sorted
    /// ascendingly by size
    pub fn into_raw_parts(self) -> Vec<Icon> {
        self.icons
    }
}

pub struct Icon {
    pub url: url::Url,
    pub raw: Option<Vec<u8>>,
    pub mime_type: Option<mime::Mime>,
    pub width: Option<u32>,
    pub height: Option<u32>
}

impl Icon {
    pub fn from_url(url: url::Url) -> Self {
        Icon {
            url: url,
            raw: None,
            mime_type: None,
            width: None,
            height: None
        }
    }

    pub fn fetch(&mut self) -> Result<()> {
        if self.raw.is_some() {
            return Ok(());
        };

        let mut response = try!(reqwest::get(self.url.clone()));
        let mut bytes: Vec<u8> = vec![];
        try!(response.read_to_end(&mut bytes));
        if !response.status().is_success() {
            return Err(ErrorKind::BadStatusCode(response).into());
        }

        let mime_type: mime::Mime = match response.headers().get::<reqwest::header::ContentType>().cloned() {
            Some(x) => x.0,
            None => return Err(ErrorKind::NoContentType(response).into())
        };
        let (better_mime_type, image_format) = match mime_type.parse_image_format() {
            Some(x) => x,
            None => return Err(ErrorKind::BadContentType(response).into())
        };
        let image = try!(image::load_from_memory_with_format(&bytes, image_format));


        self.width = Some(image.width());
        self.height = Some(image.height());
        self.raw = Some(bytes);
        self.mime_type = Some(better_mime_type);
        Ok(())
    }

    pub fn fetch_dimensions(&mut self) -> Result<()> {
        match (self.width, self.height) {
            (Some(_), Some(_)) => Ok(()),
            _ => self.fetch()
        }
    }
}
