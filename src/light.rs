use euler::DVec3;
use color::Color;
use std::f64;

const AMBIENT_PORTION : f64 = 0.01;

pub trait Lightable: LightableClone {
    fn get_direction_to(&self, destination: DVec3) -> DVec3;
    fn get_distance_to(&self, destination: DVec3) -> f64;
    fn get_intensity(&self) -> Color;
    fn get_intensity_at_distance(&self, distance: f64) -> Color;
    fn get_intensity_at_point(&self, destination: DVec3) -> Color {
        self.get_intensity_at_distance(self.get_distance_to(destination))
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
    fn get_direction_to(&self, _: DVec3) -> DVec3 {
        self.direction
    }

    fn get_distance_to(&self, _: DVec3) -> f64 {
        f64::INFINITY
    }

    fn get_intensity(&self) -> Color {
        self.color * self.power
    }

    fn get_intensity_at_distance(&self, _: f64) -> Color {
        self.get_intensity()
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
    fn get_direction_to(&self, destination: DVec3) -> DVec3 {
        (destination - self.position).normalize()
    }

    fn get_distance_to(&self, destination: DVec3) -> f64 {
        (destination - self.position).length()
    }

    fn get_intensity(&self) -> Color {
        self.color * self.power
    }

    fn get_intensity_at_distance(&self, distance: f64) -> Color {
        self.get_intensity() / (self.falloff.0 + self.falloff.1*distance + self.falloff.2*distance*distance)
    }
}