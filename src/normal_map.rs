use color::Color;
use image::RgbImage;
use geometry::SurfaceCoord;
use std::sync::Arc;

pub trait NormalMappable: NormalMappableClone {
    fn get_normal(&self, surface_coord: SurfaceCoord) -> DVec3 {
        self.calculate_normal(surface_coord, dvec3!(0.0, 0.0, 1.0))
    }
    fn calculate_normal(&self, surface_coord: SurfaceCoord, old_normal: DVec3) -> DVec3;
}

pub trait NormalMappableClone {
    fn clone_box(&self) -> Box<NormalMappable + Send + Sync>;
}

impl<T> NormalMappableClone for T
where
    T: 'static + NormalMappable + Send + Sync + Clone
{
    fn clone_box(&self) -> Box<NormalMappable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<NormalMappable + Send + Sync> {
    fn clone(&self) -> Box<NormalMappable + Send + Sync> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct NormalMap {
    normal_map: Arc<RgbImage>
    // TODO: add tiling
}

impl NormalMap {
    pub fn from_path(path: &str) -> Box<ImageTexture> {
        Box::new(NormalMap {
            image: Arc::new(image::open(path).unwrap().to_rgb()),
        })
    }
}

impl NormalMappable for NormalMap {
}