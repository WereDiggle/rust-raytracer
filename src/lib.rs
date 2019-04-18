extern crate snowflake;
extern crate image;
extern crate euler;
extern crate rand;
extern crate tobj;

pub mod color;
pub mod scene;
pub mod geometry;
pub mod shader;
pub mod primitive;
pub mod light;
pub mod multithread;
pub mod progress_tracker;
pub mod render;
pub mod composite_shape;
pub mod util;
pub mod texture;
pub mod normal_map;
pub mod asset_manager;
pub mod mesh;

use image::{RgbImage};
pub use color::*;
pub use scene::*;
pub use shader::*;
pub use primitive::*;
pub use light::*;
pub use progress_tracker::*;
pub use render::*;
pub use composite_shape::*;
pub use geometry::*;
pub use util::*;
pub use geometry::matrix::*;
pub use texture::*;
pub use normal_map::*;
pub use asset_manager::*;
pub use mesh::*;

// TODO: make this more robust, so it creates directories as well
pub fn write_to_png(img: RgbImage, file_name: &str) {
    match img.save(format!("{}.png", file_name)) {
        Ok(_) => println!("save successful"),
        Err(msg) => println!("FAILED TO SAVE: {}",msg),
    }
}