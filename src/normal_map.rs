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

    pub fn from_path(path: &str, depth: f64) -> Box<BumpMap> {
        Box::new(BumpMap {
            bump_map: Arc::new(image::open(path).unwrap().to_luma()),
            depth,
        })
    }

    fn bump_height(&self, u: u32, v: u32) -> f64 {
        // TODO: bounds checking
        (self.bump_map.get_pixel(u, v).data[0] as f64/ 255.0) * self.depth
    }
}

impl NormalMappable for BumpMap {
    fn get_normal(&self, surface_coord: SurfaceCoord) -> DVec3 {
        // We're treating each pixel of the bump map as a vertex, so width & height is one less
        let (u, v) = surface_coord.get_uv_index(self.bump_map.width()-1, self.bump_map.height()-1);

        // Get heights of vertices
        let up_left = self.bump_height(u, v+1);
        let up_right = self.bump_height(u+1, v+1);
        let bot_left = self.bump_height(u, v);
        let bot_right = self.bump_height(u+1, v);

        // Get position inside of pixel
        let (u, v) = surface_coord.get_uv_decimal(self.bump_map.width()-1, self.bump_map.height()-1);

        // Get y of normal
        let up = u*up_right + (1.0-u)*up_left;
        let bot = u*bot_right + (1.0-u)*bot_left;
        let h = up-bot;
        let h_denom = (h*h+1.0).sqrt();
        let y_vec = dvec3!(0.0, h/h_denom, 1.0/h_denom);

        // Get X of normal
        let left = v*up_left + (1.0-v)*bot_left;
        let right = v*up_right + (1.0-v)*bot_right;
        let h = right-left;
        let h_denom = (h*h+1.0).sqrt();
        let x_vec = dvec3!(h/h_denom, 0.0, 1.0/h_denom);

        let normal = (y_vec + x_vec).normalize();
        assert!(normal.length() - 1.0 < 0.00001);
        normal
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
        let (u, v) = surface_coord.get_uv_index(self.normal_map.width(), self.normal_map.height());

        let normal = self.normal_map.get_pixel(u, v);
        let normal = dvec3!(128 - normal.data[0] as i32, 128 - normal.data[1] as i32, normal.data[2]).normalize();

        normal
    }
}