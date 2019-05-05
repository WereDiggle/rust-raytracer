use std::sync::{mpsc};
use image::{RgbImage, ImageBuffer};
use euler::*;
use color::*;
use scene::*;
use geometry::*;
use light::*;
use multithread::*;
use progress_tracker::*;
use rand::prelude::*;
use std::f64::consts::PI;

// How different must neighbouring pixels be to be anti-aliased
const AA_THRESHOLD: f64 = 0.08;

// How many more rays to use for anti_aliasing
const AA_RAYS: u32 = 5;

// How many worker threads available for jobs
const NUM_THREADS: usize = 8;

// How many chunks to split the workload into
const WORKLOAD_SPLIT: u32 = 500;

// How times a ray can reflect/refract/etc through the scene
const RECURSION_DEPTH: u32 = 20;

#[derive(Clone, Copy)]
pub struct RenderConfig {
    pub num_threads: usize,
    pub workload_split: u32,
    pub anti_alias: bool,
    pub aa_threshold: f64,
    pub aa_rays: u32,
    pub recursion_depth: u32,
}

impl RenderConfig {
    pub fn default() -> RenderConfig {
        RenderConfig {
            num_threads: NUM_THREADS,
            workload_split: WORKLOAD_SPLIT,
            anti_alias: true,
            aa_threshold: AA_THRESHOLD,
            aa_rays: AA_RAYS,
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

// Takes a scene and some config options and spits out an image
pub fn render(scene: Scene,
              image_dimension: ImageDimension,
              camera_config: CameraConfig) -> RgbImage {

    let render_config = RenderConfig::default();                  
    render_with_config(scene, image_dimension, camera_config, render_config)
}

// Takes a scene and some config options and spits out an image
// render_config gives extra control over rendering settings
pub fn render_with_config(  scene: Scene,
                            image_dimension: ImageDimension,
                            camera_config: CameraConfig,
                            render_config: RenderConfig) -> RgbImage {

    let width = image_dimension.width;
    let height = image_dimension.height;

    // Calculate some values for camera configuration
    let aspect_ratio = width as f64 / height as f64;
    let fov_factor = (camera_config.fov_y.to_radians()/2.0).tan();
    let x_factor = aspect_ratio * fov_factor;

    // Set up the Basis of the camera's view
    let view_direction = (camera_config.target - camera_config.origin).normalize();
    let side = view_direction.cross(camera_config.up).normalize();
    let up = side.cross(view_direction).normalize();

    // Let's us transform points/rays from camera's perspective
    // into the scene's coordinates
    let camera_to_world_mat = dmat4!(
            side.x, side.y, side.z, 0.0,
            up.x, up.y, up.z, 0.0,
            view_direction.x, view_direction.y, view_direction.z, 0,
            camera_config.origin.x, camera_config.origin.y, camera_config.origin.z, 1,
    );

    // Handy closure to calculate a pixel's location in the scene
    let calculate_pixel_location = move |x: f64, y: f64| -> DVec3 {
        (camera_to_world_mat * dvec4!((2.0 * ((x as f64 + 0.5)/width as f64) - 1.0) * x_factor, 
                                     (1.0 - 2.0 * (y as f64 + 0.5)/height as f64) * fov_factor, 
                                      1, 
                                      1)).xyz()
    };

    // Initialization of Thread Resources
    let thread_pool = ThreadPool::new(render_config.num_threads);
    let (sender, receiver) = mpsc::channel::<(u32, Vec<(f64, Color)>)>();
    let progress_tracker = ProgressTracker::new(image_dimension);

    // Divide work into horizontal chunks of the image
    let lines_per_chunk = divide_round_up(height, render_config.workload_split);
    for chunk in 0..render_config.workload_split as u32 {

        // cloned so thread owns it's own copy of these
        let thread_sender = sender.clone();
        let thread_progress_sender = progress_tracker.get_sender();
        let thread_scene = scene.clone();

        // Each thread will run in its own little closure
        thread_pool.execute(move || {
            let mut image_chunk: Vec<(f64, Color)> = Vec::with_capacity((width * lines_per_chunk) as usize);
            for y in chunk*lines_per_chunk..height.min((chunk+1)*lines_per_chunk) {
                for x in 0..width {

                    // The actual work of ray tracing
                    let pixel_location = calculate_pixel_location(x as f64 + 0.5, y as f64 + 0.5);
                    let prime_ray = Ray::from_destination(camera_config.origin, pixel_location, render_config.recursion_depth);
                    let (distance, color) = thread_scene.cast_ray_get_distance(prime_ray);
                    image_chunk.push((distance, color));

                    // Send progress report
                    thread_progress_sender.send(ProgressMessage::Progress(1)).unwrap();
                }

                // Only send progress after every line to not overload progress track
                //thread_progress_sender.send(ProgressMessage::Progress(width)).unwrap();
            }
            thread_sender.send((chunk, image_chunk)).unwrap();
        });
    }

    // Collect completed work from worker threads
    let mut collected_chunks: Vec<Vec<(f64, Color)>> = vec![Vec::new(); render_config.workload_split as usize];
    for _ in 0..render_config.workload_split {
        let (i, line_colors) = receiver.recv().unwrap();
        collected_chunks[i as usize] = line_colors;
    }

    // put into a single vec
    let mut color_vec: Vec<(f64, Color)> = Vec::with_capacity((width * height) as usize);
    for chunk in collected_chunks.iter_mut() {
        color_vec.append(chunk);
    }

    // Useful closure for indexing our Vector<Color>
    let color_index = |x: u32, y: u32| -> usize {
        (y*width + x) as usize
    };

    // Do Anti-Aliasing
    if render_config.anti_alias {
        let eight_directions: [(i64, i64); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1,-1), (1,0), (1,1)];
        let mut aa_corrections: Vec<(u32, u32)> = Vec::new();
        for y in 1..height-1 {
            for x in 1..width-1 {

                let color_i = color_index(x, y) as i64;
                let color = color_vec[color_i as usize];

                // Check all eight neighbours of a pixel to see if it needs anti-aliasing
                for direc in eight_directions.iter() {
                    if color.1.diff(color_vec[(color_i + direc.0*width as i64 + direc.1) as usize].1) > render_config.aa_threshold  &&
                       color.0 < std::f64::INFINITY {
                        aa_corrections.push((x, y));
                        break;
                    }
                }
            }
        }

        // Initialize thread resources for anti-aliasing this time
        let (sender, receiver) = mpsc::channel::<(u32, u32, Color)>();
        let progress_sender = progress_tracker.get_sender();
        progress_sender.send(ProgressMessage::StartAA(aa_corrections.len() as u32)).unwrap();

        let theta = 2.0*PI/render_config.aa_rays as f64;

        let num_corrections = aa_corrections.len();
        let corrections_per_thread = divide_round_up(aa_corrections.len() as u32, render_config.workload_split);
        for _ in 0..render_config.workload_split {

            // Clone these for the threads to own a copy
            // TODO: there has to be a better way for threads to access this info
            let thread_sender = sender.clone();
            let thread_progress_sender = progress_tracker.get_sender();
            let thread_scene = scene.clone();

            // Each thread gets its own list of anti-aliasing corrections to complete
            // TODO: use this information more effectively in AA process
            let mut corrections: Vec<(u32, u32, f64, Color)> = Vec::new();
            for _ in 0..corrections_per_thread {
                if let Some(correction) = aa_corrections.pop() {
                    let x = correction.0;
                    let y = correction.1;
                    let color_i = color_index(x, y);
                    let (distance, color) = color_vec[color_i];
                    corrections.push((x, y, distance, color));
                }
                else {
                    break;
                }
            }

            thread_pool.execute(move || {

                let mut rng = rand::thread_rng();

                for correction in corrections.into_iter() {
                    let x = correction.0;
                    let y = correction.1;
                    let mut correction_colors: Vec<Color> = Vec::with_capacity(9);
                    correction_colors.push(correction.3);

                    let mut aa_directions: Vec<DVec2> = Vec::with_capacity(render_config.aa_rays as usize);
                    let rand_rotation = rng.gen_range(0.0, 2.0*PI);
                    for i in 0..render_config.aa_rays {
                        let x = (rand_rotation + theta * i as f64).cos();
                        let y = (rand_rotation + theta * i as f64).sin();
                        aa_directions.push(dvec2!(x, y));
                    }

                    for aa_direction in aa_directions.iter() {
                        // let corr_x = correction_dir.0 as f64;
                        // let corr_y = correction_dir.1 as f64;

                        let rand_distance: f64 = rng.gen_range(0.2, 0.5);
                        let rand_direction = *aa_direction * rand_distance;
                        let x_pos = x as f64 + 0.5 + rand_direction.x;
                        let y_pos = y as f64 + 0.5 + rand_direction.y;
                        let pixel_location = calculate_pixel_location(x_pos, y_pos);
                        let prime_ray = Ray::from_destination(camera_config.origin, pixel_location, render_config.recursion_depth);

                        if correction.2 != std::f64::INFINITY {
                            correction_colors.push(thread_scene.cast_ray(prime_ray));
                        }
                        else {
                            correction_colors.push(thread_scene.get_background_color(prime_ray));
                        }
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

        // Collect the completed anti-aliasing work from worker threads
        for _ in 0..num_corrections {
            let (x, y, color) = receiver.recv().unwrap();
            color_vec[color_index(x, y)] = (0.0, color);
        }
    }

    // Shove all those colors into an RgbImage
    make_image(image_dimension.width, image_dimension.height, color_vec.into_iter().map(|x| x.1).collect())
}

// integer division, but it rounds up
fn divide_round_up(a: u32, b:u32) -> u32 {
    (a as f32 / b as f32).ceil() as u32
}

// Convient function for making an RgbImage
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