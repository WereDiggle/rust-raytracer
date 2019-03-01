use scene::Scene;
use scene::Traceable;
use texture::TextureMappable;
use color::Color;
use euler::DVec3;
use geometry::{Intersect, Ray, SurfaceCoord};
use std::collections::HashMap;
use snowflake::ProcessUniqueId;

pub mod phong;
pub mod texture;
pub mod reflection;
pub mod translucent;

pub use self::phong::PhongShader;
pub use self::texture::TextureShader;
pub use self::reflection::ReflectionShader;
pub use self::translucent::TranslucentShader;

pub trait Shadable: ShadableClone {
    fn get_color(&self, scene: &Scene, intersect: Intersect) -> Color;

    fn get_opacity(&self) -> Color {
        Color::WHITE
    }
}

pub trait ShadableClone {
    fn clone_box(&self) -> Box<Shadable + Send + Sync>;
}

impl<T> ShadableClone for T
where
    T: 'static + Shadable + Send + Sync + Clone
{
    fn clone_box(&self) -> Box<Shadable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Shadable + Send + Sync> {
    fn clone(&self) -> Box<Shadable + Send + Sync> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct MixShader {
    shaders: Vec<Box<Shadable + Send + Sync>>,
}

impl MixShader {
    pub fn from_shaders(shaders: Vec<Box<Shadable + Send + Sync>>) -> Box<MixShader> {
        Box::new(MixShader{shaders})
    }

    pub fn add_shader(&mut self, shader: Box<Shadable + Send + Sync>) {
        self.shaders.push(shader);
    }
}

impl Shadable for MixShader {
    fn get_color(&self, scene: &Scene, intersect: Intersect) -> Color {
        self.shaders.iter().fold(Color::WHITE, |acc, x| acc*x.get_color(&scene, intersect.contributes(acc)))
    }

    fn get_opacity(&self) -> Color {
        self.shaders.iter().fold(Color::WHITE, |acc, x| acc*x.get_opacity())
    }
}

#[derive(Clone)]
pub struct CompositeShader {
    shaders: Vec<(f64, Box<Shadable + Send + Sync>)>,
}

impl CompositeShader {
    pub fn new() -> Box<CompositeShader> {
        Box::new(CompositeShader{shaders: Vec::new()})
    }

    pub fn add_shader(&mut self, weight: f64, shader: Box<Shadable + Send + Sync>) {
        self.shaders.push((weight, shader));
    }
}

impl Shadable for CompositeShader {
    fn get_color(&self, scene: &Scene, intersect: Intersect) -> Color {
        let mut total_color = Color::BLACK;
        for (weight, shader) in self.shaders.iter() {
            total_color += *weight * shader.get_color(scene, intersect.contributes(*weight*Color::WHITE));
        }
        total_color
    }

    fn get_opacity(&self) -> Color {
        let mut total_opacity = Color::BLACK; 
        for (weight, shader) in self.shaders.iter() {
            total_opacity += *weight * shader.get_opacity();
        }
        total_opacity
    }
}