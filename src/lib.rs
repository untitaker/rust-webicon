#[macro_use] extern crate quick_error;
#[macro_use] extern crate string_cache;
extern crate mime;
extern crate hyper;
extern crate url;
extern crate kuchiki;
extern crate image;

mod strategies;
mod util;

use image::GenericImage;
use strategies::Strategy;
use std::io::Read;
use util::AsImageFormat;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Hyper(error: hyper::Error) { from() }
        Io(error: std::io::Error) { from() }
        Image(error: image::ImageError) { from() }
        Other(msg: String) {
            description(msg)
        }
    }
}


pub struct IconScraper {
    pub document_url: url::Url,
    pub dom: Option<kuchiki::NodeRef>,
    pub icons: Vec<Icon>
}

impl IconScraper {
    pub fn from_url(url: url::Url) -> Self {
        IconScraper {
            document_url: url,
            dom: None,
            icons: vec![]
        }
    }

    pub fn fetch_document(&mut self) -> Result<(), Error> {
        let client = hyper::client::Client::new();
        let mut response = try!(client.get(self.document_url.clone()).send());
        let parser = try!(kuchiki::Html::from_stream(&mut response));
        self.dom = Some(parser.parse());
        Ok(())
    }

    pub fn fetch_icons(&mut self) {
        self.icons = strategies::LinkRelStrategy.get_guesses(&mut self)
            .into_iter()
            .chain(strategies::DefaultFaviconPathStrategy.get_guesses(&mut self).into_iter())
            .filter_map(|mut icon| if icon.fetch_dimensions().is_ok() { Some(icon) } else { None })
            .collect::<Vec<_>>();

        self.icons.sort_by(|a, b| {
            (a.width.unwrap() * a.height.unwrap())
                .cmp(&(b.width.unwrap() * b.height.unwrap()))
        });
    }

    pub fn at_least(self, width: u32, height: u32) -> Option<Icon> {
        self.icons
            .into_iter()
            .skip_while(|icon| icon.width.unwrap() < width || icon.height.unwrap() < height)
            .next()
    }

    pub fn largest(self) -> Option<Icon> {
        if self.icons.len() > 0 {
            Some(self.icons[self.icons.len() - 1])
        } else {
            None
        }
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

    pub fn fetch(&mut self) -> Result<(), Error> {
        let client = hyper::client::Client::new();
        let mut response = try!(client.get(self.url.clone()).send());

        let mut bytes: Vec<u8> = vec![];
        try!(response.read_to_end(&mut bytes));
        if !response.status.is_success() {
            return Err(Error::Other(format!("Bad status code: {:?}", response.status)));
        }

        let mime_type: mime::Mime = match response.headers.get::<hyper::header::ContentType>() {
            Some(x) => x.clone().0,
            None => return Err(Error::Other("No Content-Type found.".to_owned()))
        };
        let image_format = match mime_type.as_image_format() {
            Some(x) => x,
            None => return Err(Error::Other(format!("Invalid image type: {:?}", mime_type)))
        };
        let image = try!(image::load_from_memory_with_format(&bytes, image_format));


        self.width = Some(image.width());
        self.height = Some(image.height());
        self.raw = Some(bytes);
        self.mime_type = Some(mime_type);
        Ok(())
    }

    pub fn fetch_dimensions(&mut self) -> Result<(), Error> {
        match (self.width, self.height) {
            (Some(_), Some(_)) => Ok(()),
            _ => self.fetch()
        }
    }
}
