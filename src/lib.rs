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
}

impl IconScraper {
    pub fn from_url(url: url::Url) -> Self {
        IconScraper {
            document_url: url,
            dom: None
        }
    }

    pub fn fetch_document(&mut self) -> Result<(), Error> {
        let client = hyper::client::Client::new();
        let mut response = try!(client.get(self.document_url.clone()).send());
        let parser = try!(kuchiki::Html::from_stream(&mut response));
        self.dom = Some(parser.parse());
        Ok(())
    }

    pub fn icons_by_size(mut self) -> Vec<Icon> {
        let mut guesses = strategies::LinkRelStrategy.get_guesses(&mut self)
            .into_iter()
            .chain(strategies::DefaultFaviconPathStrategy.get_guesses(&mut self).into_iter())
            .filter_map(|mut icon| if icon.fetch_dimensions().is_ok() { Some(icon) } else { None })
            .collect::<Vec<_>>();

        guesses.sort_by(|a, b| {
            (a.width.unwrap() * a.height.unwrap())
                .cmp(&(b.width.unwrap() * b.height.unwrap()))
        });
        guesses
    }

    pub fn at_least(self, width: u32, height: u32) -> Option<Icon> {
        self.icons_by_size()
            .into_iter()
            .skip_while(|icon| icon.width.unwrap() < width || icon.height.unwrap() < height)
            .next()
    }
}

pub struct Icon {
    pub url: url::Url,
    pub image: Option<image::DynamicImage>,
    pub width: Option<u32>,
    pub height: Option<u32>
}

impl Icon {
    pub fn fetch(&mut self) -> Result<(), Error> {
        let client = hyper::client::Client::new();
        let mut response = try!(client.get(self.url.clone()).send());

        let mut bytes: Vec<u8> = vec![];
        try!(response.read_to_end(&mut bytes));
        if !response.status.is_success() {
            return Err(Error::Other(format!("Bad status code: {:?}", response.status)));
        }

        let image_format_opt = response.headers.get::<hyper::header::ContentType>()
            .and_then(|x| x.as_image_format());

        let image = try!(if let Some(image_format) = image_format_opt {
            image::load_from_memory_with_format(&bytes, image_format)
        } else {
            image::load_from_memory(&bytes)
        });


        self.width = Some(image.width());
        self.height = Some(image.height());
        self.image = Some(image);
        Ok(())
    }

    pub fn fetch_dimensions(&mut self) -> Result<(), Error> {
        match (self.width, self.height) {
            (Some(_), Some(_)) => Ok(()),
            _ => self.fetch()
        }
    }
}
