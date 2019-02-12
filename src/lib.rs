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

use std::sync::{mpsc};
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
const RECURSION_DEPTH: u32 = 20;

pub fn render(scene: Scene,
              image_dimension: ImageDimension,
              camera_config: CameraConfig) -> RgbImage {

    let mut image = ImageBuffer::new(image_dimension.width, image_dimension.height);
    for (.., pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgb([0, 0, 255]);
    }

    let aspect_ratio = image_dimension.width as f64 / image_dimension.height as f64;
    let fov_factor = (camera_config.fov_y.to_radians()/2.0).tan();
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
    //let (sender, receiver) = mpsc::channel::<(u32, u32, Rgb<u8>)>();
    let (sender, receiver) = mpsc::channel::<(u32, Vec<Rgb<u8>>)>();

    // TODO: divide tasks only into chunks instead of each individual ray getting a task
    let lines_per_chunk = (image_dimension.height as f32 / NUM_THREADS as f32).ceil() as u32;
    for chunk in 0..NUM_THREADS as u32 {
        let thread_sender = sender.clone();
        let thread_scene = scene.clone();
        thread_pool.execute(move || {
            for y in chunk*lines_per_chunk..image_dimension.height.min((chunk+1)*lines_per_chunk) {
                let mut image_line: Vec<Rgb<u8>> = Vec::with_capacity(image_dimension.width as usize);
                for x in 0..image_dimension.width {
                    let pixel_location = camera_to_world_mat *
                                            dvec4!((2.0 * ((x as f64 + 0.5)/image_dimension.width as f64) - 1.0) * x_factor,
                                                   (1.0 - 2.0 * (y as f64 + 0.5)/image_dimension.height as f64) * fov_factor, 
                                                    1, 
                                                    1);
                    
                    let prime_ray = Ray::from_destination(camera_config.origin, pixel_location.xyz(), RECURSION_DEPTH);

                    let color = thread_scene.cast_ray(prime_ray);

                    image_line.push(color.clamp().to_rgb());
                }
                thread_sender.send((y, image_line)).unwrap();
            }
        });
    }

    let total_num_rays = image_dimension.height * image_dimension.width;
    for progress in 0..image_dimension.height {
        let (y, line_colors) = receiver.recv().unwrap();
        for x in 0..image_dimension.width {
            image.put_pixel(x, y, line_colors[x as usize]);
        }
        // TODO: print progress
        //if progress % image_dimension.width == 0 {
        //    println!("{}/{}", progress, total_num_rays);
        //}
    }

    image
}

pub fn write_to_png(img: RgbImage, file_name: &str) {
    match img.save(format!("{}.png", file_name)) {
        Ok(_) => println!("save successful"),
        Err(msg) => println!("FAILED TO SAVE: {}",msg),
    }
}