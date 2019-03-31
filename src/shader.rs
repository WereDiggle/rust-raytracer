use scene::Scene;
use scene::Traceable;
use texture::TextureMappable;
use normal_map::NormalMappable;
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

    fn modify_intersect(&self, _: &Scene, intersect: Intersect) -> Intersect {
        intersect
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

    pub fn from_shaders(shaders: Vec<(f64, Box<Shadable + Send + Sync>)>) -> Box<CompositeShader> {
        Box::new(CompositeShader{shaders})
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

#[derive(Clone)]
pub struct ChainShader {
    shaders: Vec<Box<Shadable + Send + Sync>>,
}

impl ChainShader {
    pub fn new() -> Box<ChainShader> {
        Box::new(ChainShader{
            shaders: Vec::new()
        })
    }

    pub fn from_shaders(shaders: Vec<Box<Shadable + Send + Sync>>) -> Box<ChainShader> {
        Box::new(ChainShader{shaders})
    }

    pub fn push_shader(&mut self, shader: Box<Shadable + Send + Sync>) {
        self.shaders.push(shader);
    }
}

impl Shadable for ChainShader {
    fn get_color(&self, scene: &Scene, intersect: Intersect) -> Color {
        let intersect = self.modify_intersect(scene, intersect);

        if let Some(shader) = self.shaders.last() {
            shader.get_color(scene, intersect)
        }
        else {
            panic!("Chain Shader needs at least one shader to function")
        }
    }

    fn modify_intersect(&self, scene: &Scene, intersect: Intersect) -> Intersect {
        let mut cur_intersect = intersect;
        for shader in self.shaders.iter() {
            cur_intersect = shader.modify_intersect(scene, cur_intersect);
        }
        cur_intersect
    } 
}

#[derive(Clone)]
pub struct NormalMapShader {
    normal_map: Box<NormalMappable + Send + Sync>,
}

impl NormalMapShader {
    pub fn new(normal_map: Box<NormalMappable + Send + Sync>) -> Box<NormalMapShader> {
        Box::new(NormalMapShader {
            normal_map,
        })
    } 
}

// We basically want to edit the intersect
impl Shadable for NormalMapShader {
    fn get_color(&self, scene: &Scene, intersect: Intersect) -> Color {
        let normal = self.normal_map.calculate_normal(intersect.surface_coord, intersect.surface_normal, scene.up);
        Color::new(normal.x, normal.y, normal.z)
    }

    fn modify_intersect(&self, scene: &Scene, intersect: Intersect) -> Intersect {
        let mut intersect = intersect.clone();
        assert!(intersect.surface_normal.length() - 1.0 < 0.0001, "normal pre: {}", intersect.surface_normal);
        intersect.surface_normal = self.normal_map.calculate_normal(intersect.surface_coord, intersect.surface_normal, scene.up);
        assert!(intersect.surface_normal.length() - 1.0 < 0.0001, "normal post: {}", intersect.surface_normal);
        intersect
    }
}