use std::fs;

use ::image::{imageops::resize, load_from_memory, EncodableLayout, RgbaImage};
use iced::widget::image;

use crate::{
    enums::{Error, VideoService},
    file::{File, ThumbnailPath},
    gui::THUMBNAIL_SIZE_BIG,
    service::{ContentID, ContentIdentifier},
};

use super::Thumbnail;

impl File for Thumbnail {
    type Path = ThumbnailPath;
}
impl ContentID for Thumbnail {
    fn get_content_id(&self) -> ContentIdentifier<Self>
    where
        Self: Sized,
    {
        ContentIdentifier::new(&self.source, &self.id, crate::enums::ContentType::Thumbnail)
    }
}
impl PartialEq for Thumbnail {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.id == other.id && self.source == other.source
    }
}
impl Default for Thumbnail {
    fn default() -> Self {
        Self {
            url: Default::default(),
            id: Default::default(),
            image: None,
            source: Default::default(),
        }
    }
}
impl Thumbnail {
    /// Creates a new thumbnail
    ///
    /// The image will be None
    pub fn new(url: String, id: String, source: VideoService) -> Thumbnail {
        Self {
            url,
            id,
            source,
            image: None,
        }
    }
    /// Builds an image into the thumbnail
    pub fn with_image(mut self, img: image::Handle) -> Self {
        self.image = Some(img);
        self
    }
    /// Tests if the thumbnail has loaded image
    pub fn has_image(&self) -> bool {
        match &self.image {
            Some(_) => true,
            None => false,
        }
    }
    /// Returns an image handle, or None if it haven't been loaded
    pub fn get_image(&self) -> Option<image::Handle> {
        match &self.image {
            Some(i) => Some(i.clone()),
            None => None,
        }
    }
    /// Loads the image from the drive
    ///
    /// # Errors
    /// Will error out if the reading or creating the image from the data fails
    ///
    /// It will also error if there is no file on the drive
    pub fn load_image(&mut self) -> Result<(), Error> {
        let mut path = self.get_content_file_path();
        path.set_extension("jpg");
        let f = fs::read(path)?;
        let img = load_from_memory(f.as_bytes())?;

        let img = img.into_rgba8();
        let mut img = resize(
            &img,
            THUMBNAIL_SIZE_BIG.0 as u32,
            THUMBNAIL_SIZE_BIG.1 as u32,
            ::image::imageops::FilterType::Gaussian,
        );
        let width = img.width();
        let height = img.height();
        let img = image::Handle::from_pixels(
            width,
            height,
            img.pixels_mut().fold(Vec::new(), |mut v, p| {
                let p = p.0;
                v.append(&mut p.into());
                v
            }),
        );
        self.image = Some(img);
        Ok(())
    }
    /// Saves loaded image to drive
    ///
    /// # Errors
    /// If the thumbnail doesn't have an image then a serialization error will occur
    ///
    /// Error will also occur if saving fails
    pub fn save_image(&self) -> Result<(), Error> {
        if self.has_image() == false {
            return Err(Error::SerializationError("No Image".to_string()));
        }
        let mut path = self.get_content_file_path();
        path.set_extension("jpg");
        let data = self.image.as_ref().unwrap().data();

        let img = match data {
            iced_native::image::Data::Path(_) => unreachable!(),
            iced_native::image::Data::Bytes(b) => load_from_memory(b).unwrap().to_rgba8(),
            iced_native::image::Data::Rgba {
                width,
                height,
                pixels,
            } => RgbaImage::from_vec(*width, *height, pixels.iter().map(|x| *x).collect()).unwrap(),
        };
        let img = resize(
            &img,
            THUMBNAIL_SIZE_BIG.0 as u32,
            THUMBNAIL_SIZE_BIG.1 as u32,
            ::image::imageops::FilterType::Gaussian,
        );
        img.save(path)?;
        Ok(())
    }
    /// Updates the thumbnail with the image
    ///
    /// # Errors
    /// Error will occur if the saving the image fails
    ///
    /// Error will also occur if the URLs doesn't match
    pub fn update(&mut self, other: Thumbnail) -> Result<(), Error> {
        if self.url == other.url {
            self.image = other.image;
            self.save_image()?;
            Ok(())
        } else {
            Err(Error::Mismatch(
                "Tried saving over a thumbnail with a different url".to_string(),
            ))
        }
    }
}
