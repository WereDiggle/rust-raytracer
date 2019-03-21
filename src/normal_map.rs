use color::Color;
use image::{GrayImage, RgbImage};
use geometry::SurfaceCoord;
use geometry::matrix::*;
use std::sync::Arc;
use euler::{DVec3, dvec3, dvec4};

pub trait NormalMappable: NormalMappableClone {
    fn get_normal(&self, surface_coord: SurfaceCoord) -> DVec3;
    fn calculate_normal(&self, surface_coord: SurfaceCoord, old_normal: DVec3, up: DVec3) -> DVec3 {
        (basis(old_normal, up) * dvec4!(self.get_normal(surface_coord), 0)).xyz()
    }
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
pub struct BumpMap {
    bump_map: Arc<GrayImage>,
    pub depth: f64,
}

impl BumpMap {
    pub fn new(image: GrayImage, depth: f64) -> Box<BumpMap> {
        Box::new(BumpMap {
            bump_map: Arc::new(image),
            depth,
        })
    }
}

impl NormalMappable for BumpMap {
    fn get_normal(&self, _: SurfaceCoord) -> DVec3 {
        // tODO: copy a bunch of code from old raytracer
        // determine if lower-left or upper-right

        dvec3!(0, 0, 0)
    }
}

#[derive(Clone)]
pub struct NormalMap {
    normal_map: Arc<RgbImage>
    // TODO: add tiling
}

impl NormalMap {
    pub fn new(image: RgbImage) -> Box<NormalMap> {
        Box::new(NormalMap {
            normal_map: Arc::new(image),
        })
    }
    pub fn from_path(path: &str) -> Box<NormalMap> {
        Box::new(NormalMap {
            normal_map: Arc::new(image::open(path).unwrap().to_rgb()),
        })
    }
}

impl NormalMappable for NormalMap {
    fn get_normal(&self, surface_coord: SurfaceCoord) -> DVec3 {
        let (u, v) = surface_coord.get_uv(self.normal_map.width() as f64, self.normal_map.height() as f64);

        let normal = self.normal_map.get_pixel(u, v);
        let normal = dvec3!(128 - normal.data[0] as i32, 128 - normal.data[1] as i32, normal.data[2]).normalize();

        normal
    }
}