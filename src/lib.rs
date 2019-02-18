extern crate snowflake;
extern crate image;
extern crate euler;

pub mod color;
pub mod scene;
pub mod geometry;
pub mod shader;
pub mod primitive;
pub mod light;
pub mod multithread;
pub mod progress_tracker;
pub mod render;

use image::{RgbImage};
pub use color::*;
pub use scene::*;
pub use shader::*;
pub use primitive::*;
pub use light::*;
pub use progress_tracker::*;
pub use render::*;

pub fn write_to_png(img: RgbImage, file_name: &str) {
    match img.save(format!("{}.png", file_name)) {
        Ok(_) => println!("save successful"),
        Err(msg) => println!("FAILED TO SAVE: {}",msg),
    }
}