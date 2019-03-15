use color::Color;
use image::RgbImage;
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
pub struct NormalMap {
    normal_map: Arc<RgbImage>
    // TODO: add tiling
}

impl NormalMap {
    pub fn from_path(path: &str) -> Box<NormalMap> {
        Box::new(NormalMap {
            normal_map: Arc::new(image::open(path).unwrap().to_rgb()),
        })
    }
}

impl NormalMappable for NormalMap {
    fn get_normal(&self, surface_coord: SurfaceCoord) -> DVec3 {

        // TODO: copied this from textures. Could probably have a wrapper class around anything that deals with normal_map buffers
        // That way, I can just pass a SurfaceCoord and it would give me back a color/vec3
        let u = surface_coord.get_u();
        let v = 1.0 - surface_coord.get_v();

        let u = self.normal_map.width() as f64 * u;
        let v = self.normal_map.height() as f64 * v;

        assert!(self.normal_map.width() > 0);
        assert!(self.normal_map.height() > 0);

        // Make sure u and v are proper indices for normal_map
        let u = (u.floor() as u32).min(self.normal_map.width()-1);
        let v = (v.floor() as u32).min(self.normal_map.height()-1);

        let normal = self.normal_map.get_pixel(u, v);
        let normal = dvec3!(normal.data[0] as i32 - 128, normal.data[1] as i32 - 128, normal.data[2]).normalize();

        normal
    }
}