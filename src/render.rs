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
pub use progress_tracker::*;

const AA_THRESHOLD: f64 = 0.08;
const NUM_THREADS: usize = 8;
const RECURSION_DEPTH: u32 = 20;

#[derive(Clone, Copy)]
pub struct RenderConfig {
    pub num_threads: usize,
    pub anti_alias: bool,
    pub aa_threshold: f64,
    pub recursion_depth: u32,
}

impl RenderConfig {
    pub fn default() -> RenderConfig {
        RenderConfig {
            num_threads: NUM_THREADS,
            anti_alias: true,
            aa_threshold: AA_THRESHOLD,
            recursion_depth: RECURSION_DEPTH,
        }
    }
}

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
pub fn render(scene: Scene,
              image_dimension: ImageDimension,
              camera_config: CameraConfig) -> RgbImage {

    let render_config = RenderConfig::default();                  
    render_with_config(scene, image_dimension, camera_config, render_config)
}
pub fn render_with_config(  scene: Scene,
                            image_dimension: ImageDimension,
                            camera_config: CameraConfig,
                            render_config: RenderConfig) -> RgbImage {

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

    let calculate_pixel_location = move |x: f64, y: f64| -> DVec3 {
        (camera_to_world_mat * dvec4!((2.0 * ((x as f64 + 0.5)/width as f64) - 1.0) * x_factor, 
                                     (1.0 - 2.0 * (y as f64 + 0.5)/height as f64) * fov_factor, 
                                      1, 
                                      1)).xyz()
    };
    let thread_pool = ThreadPool::new(render_config.num_threads);
    let (sender, receiver) = mpsc::channel::<(u32, Vec<Color>)>();
    let progress_tracker = ProgressTracker::new(image_dimension);

    let lines_per_chunk = divide_round_up(height, render_config.num_threads as u32);
    for chunk in 0..render_config.num_threads as u32 {
        let thread_sender = sender.clone();
        let thread_progress_sender = progress_tracker.get_sender();
        let thread_scene = scene.clone();
        thread_pool.execute(move || {
            let mut image_line: Vec<Color> = Vec::with_capacity((width * lines_per_chunk) as usize);
            for y in chunk*lines_per_chunk..height.min((chunk+1)*lines_per_chunk) {
                for x in 0..width {
                    let pixel_location = calculate_pixel_location(x as f64 + 0.5, y as f64 + 0.5);
                    
                    let prime_ray = Ray::from_destination(camera_config.origin, pixel_location, render_config.recursion_depth);

                    let color = thread_scene.cast_ray(prime_ray);
                    //let color = cast_anti_alias_ray(&thread_scene, prime_ray);

                    image_line.push(color);
                }
                thread_progress_sender.send(ProgressMessage::Progress(width)).unwrap();
            }
            thread_sender.send((chunk, image_line)).unwrap();
        });
    }

    //let total_num_rays = image_dimension.height * image_dimension.width;
    let mut collected_chunks: Vec<Vec<Color>> = vec![Vec::new(); render_config.num_threads];
    for _ in 0..render_config.num_threads {
        let (i, line_colors) = receiver.recv().unwrap();
        collected_chunks[i as usize] = line_colors;
    }

    let mut color_vec: Vec<Color> = Vec::with_capacity((width * height) as usize);
    for chunk in collected_chunks.iter_mut() {
        color_vec.append(chunk);
    }

    let color_index = |x: u32, y: u32| -> usize {
        (y*width + x) as usize
    };

    // ANTI ALIASING
    if render_config.anti_alias {
        let eight_directions: [(i64, i64); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1,-1), (1,0), (1,1)];
        let mut aa_corrections: Vec<(u32, u32)> = Vec::new();
        for y in 1..height-1 {
            for x in 1..width-1 {
                let color_i = color_index(x, y) as i64;
                let color = color_vec[color_i as usize];
                for direc in eight_directions.iter() {
                    if color.diff(color_vec[(color_i + direc.0*width as i64 + direc.1) as usize]) > render_config.aa_threshold {
                        aa_corrections.push((x, y));
                        break;
                    }
                }
            }
        }

        let (sender, receiver) = mpsc::channel::<(u32, u32, Color)>();
        let progress_sender = progress_tracker.get_sender();
        progress_sender.send(ProgressMessage::StartAA(aa_corrections.len() as u32)).unwrap();

        let num_corrections = aa_corrections.len();
        let corrections_per_thread = divide_round_up(aa_corrections.len() as u32, render_config.num_threads as u32);
        for _ in 0..render_config.num_threads {
            let thread_sender = sender.clone();
            let thread_progress_sender = progress_tracker.get_sender();
            let thread_scene = scene.clone();
            let mut corrections: Vec<(u32, u32, Color)> = Vec::new();
            for _ in 0..corrections_per_thread {
                if let Some(correction) = aa_corrections.pop() {
                    let x = correction.0;
                    let y = correction.1;
                    let color_i = color_index(x, y);
                    corrections.push((x, y, color_vec[color_i]));
                }
                else {
                    break;
                }
            }
            thread_pool.execute(move || {
                for correction in corrections.into_iter() {
                    let x = correction.0;
                    let y = correction.1;
                    let mut correction_colors: Vec<Color> = Vec::with_capacity(9);
                    correction_colors.push(correction.2);
                    for correction_dir in eight_directions.iter() {
                        let corr_x = correction_dir.0 as f64;
                        let corr_y = correction_dir.1 as f64;

                        let pixel_location = calculate_pixel_location(x as f64 + 0.5 + (corr_x * 0.4), y as f64 + 0.5 + (corr_y * 0.4));
                        let prime_ray = Ray::from_destination(camera_config.origin, pixel_location, render_config.recursion_depth);

                        correction_colors.push(thread_scene.cast_ray(prime_ray));
                    }
                    let mut total_color = Color::BLACK;
                    let num_colors = correction_colors.len();
                    for color in correction_colors.into_iter() {
                        total_color += color;
                    }

                    thread_sender.send((x, y, total_color / num_colors as f64)).unwrap();
                    thread_progress_sender.send(ProgressMessage::AAProgress(1)).unwrap();
                }
            });
        }

        for _ in 0..num_corrections {
            let (x, y, color) = receiver.recv().unwrap();
            color_vec[color_index(x, y)] = color;
        }
    }

    make_image(image_dimension.width, image_dimension.height, color_vec)
}

fn divide_round_up(a: u32, b:u32) -> u32 {
    (a as f32 / b as f32).ceil() as u32
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