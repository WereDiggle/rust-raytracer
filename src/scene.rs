use euler::DMat4;
use color::Color;
use light::{AmbientLight, PointLight};
use geometry::{matrix, Intersect, Intersectable, Ray};
use shader::{Shadable, PhongShader};

#[derive(Clone)]
pub struct Scene {
    pub root: SceneNode,
    pub lights: Vec<PointLight>,
    pub ambient_light: AmbientLight
}

impl Scene {
    pub fn new() -> Scene {
        Scene{ root: SceneNode::new(), lights: Vec::new(), ambient_light: AmbientLight::new(Color::BLACK, 0.0)}
    }

    pub fn cast_ray(&self, ray: Ray) -> Color {
        if ray.depth > 0 {
            let intersect = self.root.trace(ray);
            if let Some(intersect) = intersect {
                return intersect.node.material.get_color(self, ray, intersect.hit_point, intersect.surface_normal);
            }
        }
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }
}

pub trait Transformable: TransformableClone {
    fn set_transform(&mut self, trans: DMat4);
    fn get_transform(&self) -> DMat4;
    fn get_inverse_transform(&self) -> DMat4;
    fn add_child(&mut self, child: Box<Transformable + Send + Sync>);
    fn add_children(&mut self, children: Vec<Box<Transformable + Send + Sync>>) {
        for child in children {
            self.add_child(child);
        }
    }
    fn trace(&self, ray: Ray) -> Option<Intersect>;
    fn partial_trace(&self, ray: Ray, max_distance: f64) -> Option<Intersect>;
}

pub trait TransformableClone {
    fn clone_box(&self) -> Box<Transformable + Send + Sync>;
}

impl<T> TransformableClone for T
where
    T: 'static + Transformable + Send + Sync + Clone
{
    fn clone_box(&self) -> Box<Transformable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Transformable + Send + Sync> {
    fn clone(&self) -> Box<Transformable + Send + Sync> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct SceneNode {
    primitive: Option<Box<Intersectable + Send + Sync>>,
    material: Box<Shadable + Send + Sync>,
    trans: DMat4,
    inv_trans: DMat4,   // Do inverse calculations only once
    children: Vec<Box<Transformable + Send + Sync>>,
}

impl SceneNode {
    pub fn new() -> SceneNode {
        let default_shader = PhongShader::new(Color::WHITE*0.5, Color::WHITE*0.5, Color::WHITE*0.1, 1.0);
        SceneNode {
            primitive: None, 
            material: Box::new(default_shader),
            trans: DMat4::identity(), 
            inv_trans: DMat4::identity(), 
            children: Vec::new()
        }
    }

    pub fn set_primitive(&mut self, primitive: Box<Intersectable + Send + Sync>) {
        self.primitive = Some(primitive);
    }

    pub fn set_material(&mut self, material: Box<Shadable + Send + Sync>) {
        self.material = material;
    }
}

impl Transformable for SceneNode {
    fn set_transform(&mut self, trans: DMat4) {
        self.trans = trans;
        self.inv_trans = trans.inverse();
    }

    fn get_transform(&self) -> DMat4 {
        self.trans
    }

    fn get_inverse_transform(&self) -> DMat4 {
        self.inv_trans
    }

    fn add_child(&mut self, child: Box<Transformable + Send + Sync>) {
        self.children.push(child);
    }

    fn trace(&self, ray: Ray) -> Option<Intersect> {
        let mut final_intersect: Option<Intersect> = None; 
        let ray = ray.transform(self.inv_trans);

        if let Some(ref primitive) = self.primitive {
            if let Some(distance) = primitive.check_intersect(ray) {
                let hit_point = ray.point_at_distance(distance);
                let surface_normal = primitive.surface_normal(hit_point);
                final_intersect = Some(Intersect::new(&self, ray, distance, hit_point, surface_normal));
            }
        }

        for child in self.children.iter() {
            if let Some(child_intersect) = child.trace(ray) {
                if let Some(intersect) = final_intersect {
                    if child_intersect.distance < intersect.distance {
                        final_intersect = Some(child_intersect);
                    }
                }
                else {
                    final_intersect = Some(child_intersect);
                }
            }
        }

        if let Some(mut intersect) = final_intersect {
            Some(intersect.transform(self.trans))
        }
        else {
            None
        }
    }

    fn partial_trace(&self, ray: Ray, max_distance: f64) -> Option<Intersect> {
        let max_distance_point = matrix::transform_point(self.inv_trans, ray.point_at_distance(max_distance));
        let ray = ray.transform(self.inv_trans);
        let max_distance = (ray.origin - max_distance_point).length();

        if let Some(ref primitive) = self.primitive {
            if let Some(distance) = primitive.check_intersect(ray) {
                if distance <= max_distance {
                    let hit_point = ray.point_at_distance(distance);
                    let surface_normal = primitive.surface_normal(hit_point);
                    return Some(Intersect::new(&self, ray, distance, hit_point, surface_normal).transform(self.trans));
                }
            }
        }

        for child in self.children.iter() {
            if let Some(mut child_intersect) = child.partial_trace(ray, max_distance) {
                if child_intersect.distance < max_distance {
                    return Some(child_intersect.transform(self.trans));
                }
            }
        }
        None
    }
}