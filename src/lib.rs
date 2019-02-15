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
use image::{RgbImage, ImageBuffer};
use euler::*;
pub use color::*;
pub use scene::*;
pub use geometry::*;
pub use shader::*;
pub use primitive::*;
pub use light::*;
pub use multithread::*;

const AA_THRESHOLD: f64 = 0.08;

#[derive(Clone, Copy)]
pub struct ImageDimension {
    pub width: u32,
    pub height: u32,
}

impl ImageDimension {
    pub fn new(width: u32, height: u32) -> ImageDimension {
        ImageDimension{width, height}
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Clone, Copy)]
pub struct CameraConfig {
    pub origin: DVec3,
    pub target: DVec3,
    pub up: DVec3,
    pub fov_y: f64,
}

impl CameraConfig {
    pub fn default() -> CameraConfig {
        CameraConfig{
            origin: dvec3!([0.0; 3]),
            target: dvec3!([0.0, 0.0, -100.0]),
            up: dvec3!([0.0, 1.0, 0.0]),
            fov_y: 90.0,
        }
    }
}

const NUM_THREADS: usize = 8;
const RECURSION_DEPTH: u32 = 20;

pub fn render(scene: Scene,
            image_dimension: ImageDimension,
            camera_config: CameraConfig) -> RgbImage {

    let width = image_dimension.width;
    let height = image_dimension.height;
    let aspect_ratio = width as f64 / height as f64;
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
    let (sender, receiver) = mpsc::channel::<(u32, Vec<Color>)>();

    let lines_per_chunk = (height as f32 / NUM_THREADS as f32).ceil() as u32;
    for chunk in 0..NUM_THREADS as u32 {
        let thread_sender = sender.clone();
        let thread_scene = scene.clone();
        thread_pool.execute(move || {
            let mut image_line: Vec<Color> = Vec::with_capacity((width * lines_per_chunk) as usize);
            for y in chunk*lines_per_chunk..height.min((chunk+1)*lines_per_chunk) {
                for x in 0..width {
                    let pixel_location = camera_to_world_mat *
                                            dvec4!((2.0 * ((x as f64 + 0.5)/width as f64) - 1.0) * x_factor,
                                                (1.0 - 2.0 * (y as f64 + 0.5)/height as f64) * fov_factor, 
                                                    1, 
                                                    1);
                    
                    let prime_ray = Ray::from_destination(camera_config.origin, pixel_location.xyz(), RECURSION_DEPTH);

                    let color = thread_scene.cast_ray(prime_ray);
                    //let color = cast_anti_alias_ray(&thread_scene, prime_ray);

                    image_line.push(color);
                }
            }
            thread_sender.send((chunk, image_line)).unwrap();
        });
    }

    //let total_num_rays = image_dimension.height * image_dimension.width;
    let mut collected_chunks: Vec<Vec<Color>> = vec![Vec::new(); NUM_THREADS];
    for _ in 0..NUM_THREADS {
        let (i, line_colors) = receiver.recv().unwrap();
        collected_chunks[i as usize] = line_colors;
        //if progress % image_dimension.width == 0 {
        //    println!("{}/{}", progress, total_num_rays);
        //}
    }

    let mut color_vec: Vec<Color> = Vec::with_capacity((width * height) as usize);
    for chunk in collected_chunks.iter_mut() {
        color_vec.append(chunk);
    }

    // ANTI ALIASING
    let mut aa_corrections: Vec<(usize, Color)> = Vec::new();
    for y in 1..height-1 {
        for x in 1..width-1 {
            let color_i = (y*width+x) as usize;
            if color_vec[color_i].diff(color_vec[color_i -width as usize -1]) > AA_THRESHOLD {
                aa_corrections.push((color_i, Color::HOT_PINK));
            }
            if color_vec[color_i].diff(color_vec[color_i -width as usize]) > AA_THRESHOLD {
                aa_corrections.push((color_i, Color::HOT_PINK));
            }
            if color_vec[color_i].diff(color_vec[color_i -width as usize +1]) > AA_THRESHOLD {
                aa_corrections.push((color_i, Color::HOT_PINK));
            }
            if color_vec[color_i].diff(color_vec[color_i -1]) > AA_THRESHOLD {
                aa_corrections.push((color_i, Color::HOT_PINK));
            }
            if color_vec[color_i].diff(color_vec[color_i +1]) > AA_THRESHOLD {
                aa_corrections.push((color_i, Color::HOT_PINK));
            }
            if color_vec[color_i].diff(color_vec[color_i +width as usize -1]) > AA_THRESHOLD {
                aa_corrections.push((color_i, Color::HOT_PINK));
            }
            if color_vec[color_i].diff(color_vec[color_i +width as usize]) > AA_THRESHOLD {
                aa_corrections.push((color_i, Color::HOT_PINK));
            }
            if color_vec[color_i].diff(color_vec[color_i +width as usize +1]) > AA_THRESHOLD {
                aa_corrections.push((color_i, Color::HOT_PINK));
            }
        }
    }

    for correction in aa_corrections.iter() {
        color_vec[correction.0] = correction.1;
    }

    make_image(image_dimension.width, image_dimension.height, color_vec)
}

fn make_image(width: u32, height: u32, colors: Vec<Color>) -> RgbImage {
    let rgb_vec: Vec<u8> = colors.into_iter().map(|x| x.clamp().to_rgb()).map(|x| vec!(x.data[0], x.data[1], x.data[2]).into_iter()).flatten().collect();
    let image: Option<RgbImage> = ImageBuffer::from_vec(width, height, rgb_vec);
    if let Some(image) = image {
        image
    }
    else {
        panic!("Could not convert rgb_vec into image");
    }
} 

pub fn cast_anti_alias_ray(scene: &Scene, ray: Ray) -> Color {
    scene.cast_ray(ray)
}


pub fn write_to_png(img: RgbImage, file_name: &str) {
    match img.save(format!("{}.png", file_name)) {
        Ok(_) => println!("save successful"),
        Err(msg) => println!("FAILED TO SAVE: {}",msg),
    }
}