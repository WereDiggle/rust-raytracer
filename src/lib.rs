extern crate image;
extern crate euler;

pub mod color;
pub mod scene;
pub mod geometry;
pub mod shader;
pub mod primitive;
pub mod light;
pub mod multithread;

use std::sync::{Mutex, Arc, mpsc};
use image::{RgbImage, Rgb, ImageBuffer};
use euler::*;
pub use color::*;
pub use scene::*;
pub use geometry::*;
pub use shader::*;
pub use primitive::*;
pub use light::*;
pub use multithread::*;

#[derive(Clone, Copy)]
pub struct ImageDimension {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
pub struct CameraConfig {
    pub origin: DVec3,
    pub target: DVec3,
    pub up: DVec3,
    pub fov_y: f64,
}

const NUM_THREADS: usize = 8;
pub fn render(scene: Scene,
              image_dimension: ImageDimension,
              camera_config: CameraConfig) -> RgbImage {

    let mut image = ImageBuffer::new(image_dimension.width, image_dimension.height);
    for (.., pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgb([0, 0, 255]);
    }

    let aspect_ratio = image_dimension.width as f64 / image_dimension.height as f64;
    let fov_factor = (aspect_ratio.to_radians()/2.0).tan();
    let x_factor = aspect_ratio * fov_factor;

    let view_direction = (camera_config.target - camera_config.origin).normalize();
    let side = view_direction.cross(camera_config.up).normalize();
    let up = side.cross(view_direction).normalize();

    let camera_to_world_mat = dmat4!(
            side.x, side.y, side.z, 0.0,
            up.x, up.y, up.z, 0.0,
            view_direction.x, view_direction.y, view_direction.z, 0,
            camera_config.origin.x, camera_config.origin.y, camera_config.origin.z, 1,
    );

    let thread_pool = ThreadPool::new(NUM_THREADS);
    let (sender, receiver) = mpsc::channel::<(u32, u32, Rgb<u8>)>();

    for y in 0..image_dimension.height {
        for x in 0..image_dimension.width {
            let thread_sender = sender.clone();
            let thread_scene = scene.clone();
            thread_pool.execute(move || {
                let pixel_location = camera_to_world_mat *
                                     dvec4!((2.0 * ((x as f64 + 0.5)/image_dimension.width as f64) - 1.0) * x_factor,
                                            (1.0 - 2.0 * (y as f64 + 0.5)/image_dimension.height as f64) * fov_factor, 
                                            1, 
                                            1);
                
                let prime_ray = Ray::new_destination(camera_config.origin, pixel_location.xyz());

                let color = thread_scene.cast_ray(prime_ray);

                thread_sender.send((x, y, color.clamp().to_rgb())).unwrap();
            });
        }
    }

    for _ in 0..image_dimension.height*image_dimension.width {
        let (x, y, color) = receiver.recv().unwrap();
        image.put_pixel(x, y, color);
    }

    image
}

pub fn write_to_png(img: RgbImage, file_name: &str) {
    match img.save(format!("{}.png", file_name)) {
        Ok(_) => println!("save successful"),
        Err(msg) => println!("FAILED TO SAVE: {}",msg),
    }
}