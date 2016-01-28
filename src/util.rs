use mime::{Mime, TopLevel, SubLevel};
use image;

// XXX: Move into Piston?
pub trait AsImageFormat {
    fn parse_image_format(&self) -> Option<(Mime, image::ImageFormat)>;
}

impl AsImageFormat for Mime {
    fn parse_image_format(&self) -> Option<(Mime, image::ImageFormat)> {
        Some(match *self {
            Mime(TopLevel::Image, SubLevel::Png, _) => (self.clone(), image::ImageFormat::PNG),
            Mime(TopLevel::Image, SubLevel::Jpeg, _) => (self.clone(), image::ImageFormat::JPEG),
            Mime(TopLevel::Image, SubLevel::Gif, _) => (self.clone(), image::ImageFormat::GIF),
            Mime(_, SubLevel::Ext(ref i), _) if i == "x-icon" || i == "vnd.microsoft.icon" => (
                Mime(TopLevel::Image, SubLevel::Ext("x-icon".into()), vec![]),
                image::ImageFormat::ICO
            ),
            _ => return None
        })
    }
}
