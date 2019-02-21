use euler::DMat4;
use color::Color;
use light::{Lightable, AmbientLight};
use geometry::{matrix, NodeIntersect, Intersectable, Transformable, TransformComponent, Ray};
use shader::{Shadable, PhongShader};
use snowflake::ProcessUniqueId;
use image::{RgbImage, ImageBuffer};
use std::f64::consts::PI;
use std::sync::Arc;

#[derive(Clone)]
pub struct Scene {
    pub root: Box<Traceable + Send + Sync>,
    pub lights: Vec<Box<Lightable + Send + Sync>>,
    pub ambient_light: AmbientLight,
    pub background: Arc<RgbImage>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene{ root: Box::new(SceneNode::new()), lights: Vec::new(), ambient_light: AmbientLight::new(Color::BLACK, 0.0), background: Arc::new(ImageBuffer::new(1, 1))}
    }

    pub fn cast_ray(&self, ray: Ray) -> Color {
        if ray.depth > 0 {
            let node_intersect = self.root.trace(ray);
            if let Some(node_intersect) = node_intersect {
                return node_intersect.shader.get_color(self, ray, node_intersect.get_hit_point(), node_intersect.get_surface_normal());
            }
        }
        self.get_background_color(ray)
    }

    pub fn cast_ray_get_distance(&self, ray: Ray) -> (f64, Color) {
        if ray.depth > 0 {
            let node_intersect = self.root.trace(ray);
            if let Some(node_intersect) = node_intersect {
                return (node_intersect.get_distance(), 
                        node_intersect.shader.get_color(self, ray, node_intersect.get_hit_point(), node_intersect.get_surface_normal()));
            }
        }
        (0.0, self.get_background_color(ray))
    }

    pub fn add_light(&mut self, light: Box<Lightable + Send + Sync>) {
        self.lights.push(light);
    }

    pub fn set_background_from_path(&mut self, file_path: &str) {
        self.background = Arc::new(image::open(file_path).unwrap().to_rgb());
    }

    fn get_background_color(&self, ray: Ray) -> Color {
        let azimuth = ray.direction.z.atan2(ray.direction.x);
        let elevation = ray.direction.y.asin();

        let u = (azimuth/2.0)/PI + 0.5;
        let v = elevation/PI + 0.5;
        assert!(u >= 0.0 && u <= 1.0);
        assert!(v >= 0.0 && v <= 1.0);
        let v = 1.0 - v;

        let u = self.background.width() as f64 * u;
        let v = self.background.height() as f64 * v;

        assert!(self.background.width() > 0);
        assert!(self.background.height() > 0);

        let u = (u.floor() as u32).min(self.background.width()-1);
        let v = (v.floor() as u32).min(self.background.height()-1);

        Color::from_rgb(self.background.get_pixel(u, v))
    }
}

pub trait Traceable: TraceableClone {
    fn get_id(&self) -> ProcessUniqueId;
    fn add_child(&mut self, child: Box<Traceable + Send + Sync>);
    fn add_children(&mut self, children: Vec<Box<Traceable + Send + Sync>>) {
        for child in children {
            self.add_child(child);
        }
    }
    fn trace(&self, ray: Ray) -> Option<NodeIntersect>;
    fn partial_trace_until_distance(&self, ray: Ray, max_distance: f64) -> Option<NodeIntersect>;
    fn total_trace_until_distance(&self, ray: Ray, max_distance: f64) -> Vec<NodeIntersect>;
}

pub trait TraceableClone {
    fn clone_box(&self) -> Box<Traceable + Send + Sync>;
}

impl<T> TraceableClone for T
where
    T: 'static + Traceable + Send + Sync + Clone
{
    fn clone_box(&self) -> Box<Traceable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Traceable + Send + Sync> {
    fn clone(&self) -> Box<Traceable + Send + Sync> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct SceneNode {
    id: ProcessUniqueId,
    primitive: Option<Box<Intersectable + Send + Sync>>,
    material: Box<Shadable + Send + Sync>,
    transform: TransformComponent,
    children: Vec<Box<Traceable + Send + Sync>>,
}

impl SceneNode {
    
    pub fn new() -> SceneNode {
        let default_shader = PhongShader::new(Color::WHITE*0.5, Color::WHITE*0.5, Color::WHITE*0.1, 1.0);
        SceneNode {
            id: ProcessUniqueId::new(),
            primitive: None, 
            material: Box::new(default_shader),
            transform: TransformComponent::new(DMat4::identity()),
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
        self.transform.set_transform(trans);
    }

    fn get_transform(&self) -> DMat4 {
        self.transform.get_transform()
    }

    fn get_inverse_transform(&self) -> DMat4 {
        self.transform.get_inverse_transform()
    }
}

impl Traceable for SceneNode {
    fn get_id(&self) -> ProcessUniqueId {
        self.id
    }

    fn add_child(&mut self, child: Box<Traceable + Send + Sync>) {
        self.children.push(child);
    }

    fn trace(&self, ray: Ray) -> Option<NodeIntersect> {
        let mut final_node_intersect: Option<NodeIntersect> = None; 
        let ray = ray.transform(self.transform.get_inverse_transform());

        if let Some(ref primitive) = self.primitive {
            if let Some(intersect) = primitive.get_closest_intersect(ray) {
                final_node_intersect = Some(NodeIntersect::new(self.id, &(*self.material), intersect));
            }
        }

        for child in self.children.iter() {
            if let Some(child_node_intersect) = child.trace(ray) {
                if let Some(node_intersect) = final_node_intersect {
                    if child_node_intersect.get_distance() < node_intersect.get_distance() {
                        final_node_intersect = Some(child_node_intersect);
                    }
                }
                else {
                    final_node_intersect = Some(child_node_intersect);
                }
            }
        }

        if let Some(intersect) = final_node_intersect {
            Some(intersect.transform(self.transform.get_transform()))
        }
        else {
            None
        }
    }

    fn partial_trace_until_distance(&self, ray: Ray, max_distance: f64) -> Option<NodeIntersect> {
        let max_distance_point = matrix::transform_point(self.transform.get_inverse_transform(), ray.point_at_distance(max_distance));
        let ray = ray.transform(self.transform.get_inverse_transform());
        let max_distance = (ray.origin - max_distance_point).length();

        if let Some(ref primitive) = self.primitive {
            if let Some(intersect) = primitive.get_closest_intersect(ray) {
                if intersect.distance <= max_distance {
                    return Some(NodeIntersect::new(self.id, &(*self.material), intersect).transform(self.transform.get_transform()));
                }
            }
        }

        for child in self.children.iter() {
            if let Some(mut child_node_intersect) = child.partial_trace_until_distance(ray, max_distance) {
                if child_node_intersect.get_distance() < max_distance {
                    return Some(child_node_intersect.transform(self.transform.get_transform()));
                }
            }
        }
        None
    }

    fn total_trace_until_distance(&self, ray: Ray, max_distance: f64) -> Vec<NodeIntersect> {
        let ray = ray.transform(self.transform.get_inverse_transform());
        let max_distance_point = matrix::transform_point(self.transform.get_inverse_transform(), ray.point_at_distance(max_distance));
        let max_distance = (ray.origin - max_distance_point).length();

        let mut all_intersects: Vec<NodeIntersect> = Vec::new();
        if let Some(ref primitive) = self.primitive {
            let intersects: Vec<NodeIntersect> = primitive.get_all_intersects(ray)
                                                          .into_iter()
                                                          .filter( |inter| inter.distance <= max_distance )
                                                          .map( |inter| NodeIntersect::new(self.id, &(*self.material), inter) )
                                                          .collect();
            all_intersects.extend(intersects);
        }

        for child in self.children.iter() {
            let child_node_intersects = child.total_trace_until_distance(ray, max_distance);
            // TODO: merge sort here instead of leaving it to the end
            all_intersects.extend(child_node_intersects);
        }
        // transform all intersects in all_intersects
        all_intersects.iter().map(|sect| sect.transform(self.transform.get_transform())).collect()
    }
}