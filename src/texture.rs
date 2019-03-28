use color::Color;
use image::RgbImage;
use geometry::SurfaceCoord;
use std::sync::Arc;

pub trait TextureMappable: TextureMappableClone {
    fn get_color(&self, surface_coord: SurfaceCoord) -> Color;
}

pub trait TextureMappableClone {
    fn clone_box(&self) -> Box<TextureMappable + Send + Sync>;
}

impl<T> TextureMappableClone for T
where
    T: 'static + TextureMappable + Send + Sync + Clone
{
    fn clone_box(&self) -> Box<TextureMappable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<TextureMappable + Send + Sync> {
    fn clone(&self) -> Box<TextureMappable + Send + Sync> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    image: Arc<RgbImage>
    // TODO: add tiling
}

impl ImageTexture {
    pub fn new(image: RgbImage) -> Box<ImageTexture> {
        Box::new(ImageTexture {
            image: Arc::new(image),
        })
    }

    pub fn from_path(path: &str) -> Box<ImageTexture> {
        Box::new(ImageTexture {
            image: Arc::new(image::open(path).unwrap().to_rgb()),
        })
    }
}

impl TextureMappable for ImageTexture {
    fn get_color(&self, surface_coord: SurfaceCoord) -> Color {
        let (u, v) = surface_coord.get_uv_index(self.image.width()-1, self.image.height()-1);

        let up_left = Color::from_rgb(self.image.get_pixel(u, v));
        let up_right = Color::from_rgb(self.image.get_pixel(u+1, v));
        let bot_left = Color::from_rgb(self.image.get_pixel(u, v+1));
        let bot_right = Color::from_rgb(self.image.get_pixel(u+1, v+1));

        let (u, v) = surface_coord.get_uv_decimal(self.image.width()-1, self.image.height()-1);

        // TODO: try non-linear weights, want corners to have more significance

        // weighted average based on inter-pixel position
        up_left * (1.0-u)*v +
        up_right * u*v +
        bot_left * (1.0-u)*(1.0-v) +
        bot_right * u*(1.0-v)
    }
}