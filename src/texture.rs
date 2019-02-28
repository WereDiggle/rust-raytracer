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
    pub fn from_path(path: &str) -> Box<ImageTexture> {
        Box::new(ImageTexture {
            image: Arc::new(image::open(path).unwrap().to_rgb()),
        })
    }
}

impl TextureMappable for ImageTexture {
    fn get_color(&self, surface_coord: SurfaceCoord) -> Color {
        let u = surface_coord.get_u();
        let v = 1.0 - surface_coord.get_v();

        let u = self.image.width() as f64 * u;
        let v = self.image.height() as f64 * v;

        assert!(self.image.width() > 0);
        assert!(self.image.height() > 0);

        // Make sure u and v are proper indices for image
        let u = (u.floor() as u32).min(self.image.width()-1);
        let v = (v.floor() as u32).min(self.image.height()-1);

        Color::from_rgb(self.image.get_pixel(u, v))
    }
}