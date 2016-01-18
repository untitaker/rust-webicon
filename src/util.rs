use mime::{Mime, TopLevel, SubLevel};
use image;

// XXX: Move into Piston?
pub trait AsImageFormat {
    fn as_image_format(&self) -> Option<image::ImageFormat>;
}

impl AsImageFormat for Mime {
    fn as_image_format(&self) -> Option<image::ImageFormat> {
        Some(match *self {
            Mime(TopLevel::Image, SubLevel::Png, _) => image::ImageFormat::PNG,
            Mime(TopLevel::Image, SubLevel::Jpeg, _) => image::ImageFormat::JPEG,
            Mime(TopLevel::Image, SubLevel::Gif, _) => image::ImageFormat::GIF,
            Mime(TopLevel::Image, SubLevel::Ext(ref i), _) if i == "icon" => image::ImageFormat::ICO,
            _ => return None
        })
    }
}
