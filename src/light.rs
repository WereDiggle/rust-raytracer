use euler::{dvec3, DVec3, dvec2, DVec2};
use color::Color;
use std::f64;
use scene::Scene;
use geometry::{Ray, Intersect};
use std::sync::Arc;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

const AMBIENT_PORTION : f64 = 0.01;

pub trait Lightable: LightableClone {
    fn get_sample(&self) -> Vec<DVec3>;

    fn get_intensity(&self, distance: f64) -> Color;

    fn get_illums_at(&self, scene: &Scene, intersect: Intersect) -> Vec<Illum> {

        let mut ret_vec: Vec<Illum> = vec!();
        let samples = self.get_sample();
        for light_pos in samples.iter() {

            let hit_to_light = *light_pos - intersect.hit_point;
            let light_direction = hit_to_light.normalize();
            let surface_dot = light_direction.dot(intersect.surface_normal);
            if surface_dot <= 0.0 {
                ret_vec.push(Illum::Unlit);
                continue;
            }

            let shadow_ray = Ray::new(intersect.hit_point, light_direction, 1);
            let light_distance = hit_to_light.length();

            if let Some(_) = scene.root.partial_trace_until_distance(shadow_ray, light_distance) {
                ret_vec.push(Illum::Unlit);
            }
            else {
                let intensity = self.get_intensity(light_distance) / samples.len() as f64;
                ret_vec.push(Illum::Lit{surface_dot, light_direction, intensity});
            }
        }
        ret_vec
    }
}

pub trait LightableClone {
    fn clone_box(&self) -> Box<Lightable + Send + Sync>;
}

impl<T> LightableClone for T
where
    T: 'static + Lightable + Send + Sync + Clone
{
    fn clone_box(&self) -> Box<Lightable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Lightable + Send + Sync> {
    fn clone(&self) -> Box<Lightable + Send + Sync> {
        self.clone_box()
    }
}


#[derive(Clone)]
pub struct AmbientLight {
    pub color: Color,
    pub power: f64,
}

impl AmbientLight {
    pub fn new(color: Color, power: f64) -> AmbientLight {
        AmbientLight{color, power}
    }

    pub fn color_intensity(&self) -> Color {
        self.color * self.power
    }
}

#[derive(Clone)]
pub struct DirectionLight {
    pub direction: DVec3,
    pub color: Color,
    pub power: f64,
}

impl DirectionLight {
    pub fn new(direction: DVec3, color: Color, power: f64) -> DirectionLight {
        DirectionLight{direction: direction.normalize(), color, power}
    }
}

impl Lightable for DirectionLight {
    fn get_sample(&self) -> Vec<DVec3> {
        vec!()
    }

    fn get_intensity(&self, _: f64) -> Color {
        self.color * self.power
    }

    fn get_illums_at(&self, scene: &Scene, intersect: Intersect) -> Vec<Illum> {
        let surface_dot = self.direction.dot(intersect.surface_normal);
        if surface_dot <= 0.0 {
            return vec!(Illum::Unlit);
        }

        let shadow_ray = Ray::new(intersect.hit_point, -1.0*self.direction, 1);

        if let Some(_) = scene.root.trace(shadow_ray) {
            vec!(Illum::Unlit)
        }
        else {
            let intensity = self.get_intensity(0.0);
            vec!(Illum::Lit{surface_dot, light_direction: -1.0*self.direction, intensity})
        }
    }
}

#[derive(Clone)]
pub struct PointLight {
    pub position: DVec3,
    pub color: Color,
    pub power: f64,
    pub falloff: (f64, f64, f64),
}

impl PointLight {
    pub fn new(position: DVec3, color: Color, power: f64, falloff: (f64, f64, f64)) -> PointLight {
        PointLight{position, color: color.normalize(), power, falloff}
    }

    pub fn color_intensity(&self) -> Color {
        self.color * self.power
    }
}

impl Lightable for PointLight {
    fn get_sample(&self) -> Vec<DVec3> {
        vec!(self.position)
    }

    fn get_intensity(&self, distance: f64) -> Color {
        self.power * self.color / (self.falloff.0 + self.falloff.1*distance + self.falloff.2*distance*distance)
    }
}


#[derive(Clone)]
pub struct SquareLight {
    // TODO: can probably just store power and color as one
    pub position: DVec3,
    pub size: f64,
    pub color: Color,
    pub power: f64,
    pub falloff: (f64, f64, f64),

    inv_area: f64,
}

pub enum Illum {
    Lit {
        surface_dot: f64,
        light_direction: DVec3,
        intensity: Color,
    },
    Unlit,
}

impl SquareLight {
    pub fn new(position: DVec3, size: f64, color: Color, power: f64, falloff: (f64, f64, f64)) -> Box<SquareLight> {
        Box::new(SquareLight{position, 
                             size, 
                             color: color.normalize(), 
                             power, 
                             falloff,
                             inv_area: 1.0/(size*size)})
    }

    pub fn color_intensity(&self) -> Color {
        self.color * self.power
    }

    const SAMPLE_RATE: usize = 400;
    const SUB_DEPTH: usize = 5;

    fn subdivide_points(&self, min: DVec2, max: DVec2, depth: usize) -> Vec<DVec3> {
        let change = max - min;
        let mut rng = rand::thread_rng();
        if false {
            let change = change*0.1;
            let r = change * 0.5;
            let low = min+change*0.5;
            let mut ret_vec: Vec<DVec3> = Vec::with_capacity(8);
            for i in 0..10 {
                for j in 0..10 {
                    ret_vec.push(self.position + dvec3!(low.x+(i as f64 * change.x)+rng.gen_range(-r.x, r.x), 
                                                        0.0, 
                                                        low.y+(j as f64 * change.y)+rng.gen_range(-r.x, r.x)));
                }
            }
            ret_vec
        }
        else {
            let low = min + change * 0.25;
            let high = min + change * 0.75;
            let left_high = dvec2!(low.x, low.y + change.y*0.5);
            let right_low = dvec2!(low.x + change.x*0.5, low.y);
            let r = change * 0.25;

            vec!(
                self.position + dvec3!(left_high.x+rng.gen_range(-r.x, r.x), 0.0, left_high.y+rng.gen_range(-r.y, r.y)),
                self.position + dvec3!(high.x+rng.gen_range(-r.x, r.x), 0.0, high.y+rng.gen_range(-r.y, r.y)),
                self.position + dvec3!(low.x+rng.gen_range(-r.x, r.x), 0.0, low.y+rng.gen_range(-r.y, r.y)),
                self.position + dvec3!(right_low.x+rng.gen_range(-r.x, r.x), 0.0, right_low.y+rng.gen_range(-r.y, r.y)),
            )
        }
    }

    fn guess_illums(&self, min: DVec2, max: DVec2, intersect: &Intersect, depth: usize, area_porp: f64) -> Vec<Illum> {
        let num_illums = (4 as usize).pow(depth as u32);
        let area_porp = 4.0 * area_porp / (num_illums as f64);
        let mut ret_vec: Vec<Illum> = Vec::with_capacity(num_illums);
        let between_x = Uniform::from(min.x..max.x);
        let between_y = Uniform::from(min.y..max.y);
        let mut rng = rand::thread_rng();
        for _ in 0..num_illums {
            let point = self.position + dvec3!(between_x.sample(&mut rng), 0.0, between_y.sample(&mut rng));
            let hit_to_light = point - intersect.hit_point;
            let light_distance = hit_to_light.length();
            let light_direction = hit_to_light / light_distance;
            let surface_dot = light_direction.dot(intersect.surface_normal);
            let intensity = self.get_intensity(light_distance) * area_porp;
            ret_vec.push(
                Illum::Lit {
                    surface_dot,
                    light_direction,
                    intensity,
                }
            )
        }
        ret_vec
    }

    // Experimental area lighting shortcut
    fn subdivide_illumination(&self, min: DVec2, max: DVec2, scene: &Scene, intersect: &Intersect, depth: usize) -> Vec<Illum> {
        let mut sample_illums: Vec<Illum> = Vec::with_capacity(4);
        let sample_points = self.subdivide_points(min, max, depth);
        let area_porp = (max.x - min.x)*(max.y - min.y)*self.inv_area*0.25;
        let mut lit_count: usize = 0;
        for light_pos in sample_points.iter() {

            let hit_to_light = *light_pos - intersect.hit_point;
            let light_direction = hit_to_light.normalize();
            let surface_dot = light_direction.dot(intersect.surface_normal);
            if surface_dot <= 0.0 {
                sample_illums.push(Illum::Unlit);
                continue;
            }

            let shadow_ray = Ray::new(intersect.hit_point, light_direction, 1);
            let light_distance = hit_to_light.length();

            if let Some(_) = scene.root.partial_trace_until_distance(shadow_ray, light_distance) {
                sample_illums.push(Illum::Unlit);
            }
            else {
                let intensity = self.get_intensity(light_distance) * area_porp;
                sample_illums.push(Illum::Lit{surface_dot, light_direction, intensity});
                lit_count+=1;
            }
        }

        // Sub division is uniform
        if depth == 1 || lit_count == 0 {
            sample_illums
        }
        else if depth > 1 && lit_count == sample_points.len() {
            self.guess_illums(min, max, intersect, depth, area_porp)
        }
        // TODO: figure out how to use both sample_illums and total_illums
        else {
            let mut total_illums: Vec<Illum> = vec!();
            let change = (max - min)*0.5;
            let left = dvec2!(min.x, min.y+change.y);
            let mid = min + change;
            let right = dvec2!(max.x, max.y-change.y);
            let top = dvec2!(max.x-change.x, max.y);
            let bot = dvec2!(min.x+change.x, min.y);
            total_illums.append(&mut self.subdivide_illumination(left, top, scene, intersect, depth-1));
            total_illums.append(&mut self.subdivide_illumination(mid, max, scene, intersect, depth-1));
            total_illums.append(&mut self.subdivide_illumination(min, mid, scene, intersect, depth-1));
            total_illums.append(&mut self.subdivide_illumination(bot, right, scene, intersect, depth-1));
            total_illums
        }
    }

    fn trace_sample_rays(&self, scene: &Scene, intersect: &Intersect, num_rays: usize) -> (usize, Vec<Illum>) {
        let mut ret_vec: Vec<Illum> = Vec::with_capacity(num_rays);
        let between_x = Uniform::from(-self.size*0.5..self.size*0.5);
        let between_y = Uniform::from(-self.size*0.5..self.size*0.5);
        let mut rng = rand::thread_rng();
        let mut count: usize = 0;
        for i in 0..num_rays {
            let sample_point = self.position + dvec3!(between_x.sample(&mut rng), 0.0, between_y.sample(&mut rng));
            let ray_vector = sample_point - intersect.hit_point;
            let light_distance = ray_vector.length();
            let light_direction = ray_vector / light_distance;
            let ray = Ray::new(intersect.hit_point, light_direction, 1);            
            if let Some(_) = scene.root.partial_trace_until_distance(ray, light_distance) {
                ret_vec.push(Illum::Unlit);
            }
            else {
                ret_vec.push(Illum::Lit{
                    surface_dot: light_direction.dot(intersect.surface_normal),
                    light_direction,
                    intensity: self.get_intensity(light_distance),  // We'll adjust intensity afterwards
                });
                count+=1;
            }
        }
        (count, ret_vec)
    }

    const START_SAMPLE: usize = 32;
    const PROG_DEPTH: usize = 4;
    const PROG_THRESH: f64 = 0.05;

    fn progressive_random_illumination(&self, scene: &Scene, intersect: &Intersect) -> Vec<Illum> {
        let mut ret_vec: Vec<Illum> = Vec::with_capacity(SquareLight::START_SAMPLE);
        let mut cur_samples = SquareLight::START_SAMPLE;
        let mut cur_thresh = 0;
        for depth in 0..SquareLight::PROG_DEPTH {
            let (num_lit, mut illums) = self.trace_sample_rays(scene, intersect, cur_samples);
            ret_vec.append(&mut illums);
            if num_lit < cur_thresh {
                break;
            }
            cur_samples *= 2;
            cur_thresh = num_lit*2;
        }
        let intensity_porp = 1.0 / ret_vec.len() as f64;
        ret_vec.into_iter().map(|mut x| {
            if let Illum::Lit{surface_dot, light_direction, ref mut intensity} = x {
                *intensity = *intensity * intensity_porp;
            }
            x
        }).collect()
    }
}

impl Lightable for SquareLight {
    fn get_sample(&self) -> Vec<DVec3> {
        let mut ret_vec: Vec<DVec3> = vec!();
        let between = Uniform::from(-self.size*0.5..self.size*0.5);
        let mut rng = rand::thread_rng();
        for i in 0..SquareLight::SAMPLE_RATE {
            ret_vec.push(self.position + dvec3!(between.sample(&mut rng), 0, between.sample(&mut rng)));
        }
        ret_vec
    }

    fn get_intensity(&self, distance: f64) -> Color {
        use std::f64::consts::PI;
        self.color * self.power / (4.0*PI*distance*distance)
    }

    fn get_illums_at(&self, scene: &Scene, intersect: Intersect) -> Vec<Illum> {
        let half_size: f64 = self.size*0.5;
        self.subdivide_illumination(dvec2!(-half_size, -half_size), dvec2!(half_size, half_size), scene, &intersect, SquareLight::SUB_DEPTH)
        //self.progressive_random_illumination(scene, &intersect)
    }
}

