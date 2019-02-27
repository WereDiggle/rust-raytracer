use scene::Scene;
use scene::Traceable;
use color::Color;
use euler::DVec3;
use geometry::Ray;
use std::collections::HashMap;
use snowflake::ProcessUniqueId;

pub trait Shadable: ShadableClone {
    fn get_color(&self, scene: &Scene, ray: Ray, hit_point: DVec3, surface_normal: DVec3) -> Color;

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
pub struct CompositeShader {
    shaders: Vec<(f64, Box<Shadable + Send + Sync>)>,
}

impl CompositeShader {
    pub fn new() -> CompositeShader {
        CompositeShader{shaders: Vec::new()}
    }

    pub fn add_shader(&mut self, weight: f64, shader: Box<Shadable + Send + Sync>) {
        self.shaders.push((weight, shader));
    }
}

impl Shadable for CompositeShader {
    fn get_color(&self, scene: &Scene, ray: Ray, hit_point: DVec3, surface_normal: DVec3) -> Color {
        let mut total_color = Color::BLACK;
        for (weight, shader) in self.shaders.iter() {
            total_color += *weight * shader.get_color(scene, ray.contributes(*weight*Color::WHITE), hit_point, surface_normal);
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
pub struct PhongShader {
    diffuse: Color,
    specular: Color,
    ambient: Color,
    shininess: f64,
}

impl PhongShader {
    pub fn new(diffuse: Color, specular: Color, ambient: Color, shininess: f64) -> PhongShader {
        PhongShader{diffuse, specular, ambient, shininess}
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Hit {
    Enter,
    Exit,
}

fn total_light_blocked(opacity: Color, enter: f64, exit: f64) -> Color {
    let total_distance = exit - enter;
    let total_blocked = opacity * total_distance;
    total_blocked
}

impl Shadable for PhongShader {
    fn get_color(&self, scene: &Scene, ray: Ray, hit_point: DVec3, surface_normal: DVec3) -> Color {
        assert!(surface_normal.length() - 1.0 < 0.000001);
        let mut total_color = self.ambient * scene.ambient_light.color_intensity();
        for light in &scene.lights {
            let light_direction = -1.0 * light.get_direction_to(hit_point);
            let light_surface_dot = light_direction.dot(surface_normal);
            if light_surface_dot <= 0.0 {
                continue;
            }

            let shadow_ray = Ray::new(hit_point, light_direction, 1);
            let light_distance = light.get_distance_to(hit_point);

            let mut light_through = Color::WHITE;
            let mut shadow_intersects = scene.root.total_trace_until_distance(shadow_ray, light_distance);
            shadow_intersects.sort_by(|a, b| a.get_distance().partial_cmp(&b.get_distance()).unwrap());

            let mut shadow_map: HashMap<ProcessUniqueId, (Color, Vec<(Hit, f64)>) > = HashMap::new();

            for shadow_intersect in shadow_intersects {
                shadow_map.entry(shadow_intersect.hit_id).or_insert((shadow_intersect.shader.get_opacity(), Vec::new()));
                if let Some((_, vec)) = shadow_map.get_mut(&shadow_intersect.hit_id) {
                    let dot = shadow_intersect.get_surface_normal().dot(shadow_intersect.get_ray().direction);
                    let (hit_type, mut last_hit_type) = if dot < 0.0 {(Hit::Enter, Hit::Exit)} else {(Hit::Exit, Hit::Enter)}; 
                    if let Some(prev) = vec.last() {
                        last_hit_type = prev.0;
                    }

                    if hit_type != last_hit_type {
                        vec.push((hit_type, shadow_intersect.get_distance())); 
                    }
                }
            }

            for (_, (opacity, vec)) in shadow_map {
                assert!(vec.len() > 0);
                let mut vec_iter = vec.iter();
                while light_through.or_greater(Color::BLACK) {
                    let mut enter_distance: f64 = 0.0;
                    let mut exit_distance: f64 = light_distance;

                    if let Some((hit_type, distance)) = vec_iter.next() {
                        match hit_type {
                            Hit::Enter => enter_distance = *distance,
                            Hit::Exit => {
                                exit_distance = *distance;
                                light_through -= total_light_blocked(opacity, enter_distance, exit_distance);
                                continue;
                            },
                        }
                    }
                    else {
                        break;
                    }

                    // We only reach this far if last hit was a Hit::Enter
                    if let Some((hit_type, distance)) = vec_iter.next() {
                        match hit_type {
                            Hit::Enter => panic!("Got a Hit::Enter after Hit::Enter"),
                            Hit::Exit => {
                                exit_distance = *distance;
                            },
                        }
                    }
                    light_through -= total_light_blocked(opacity, enter_distance, exit_distance);
                }
            }
            light_through = light_through.clamp();

            let reflection_direction = (2.0*light_surface_dot*surface_normal - light_direction).normalize();
            let specular_factor = reflection_direction.dot(-1.0*ray.direction).max(0.0).powf(self.shininess) / light_surface_dot;
            let phong_color = self.specular * specular_factor + self.diffuse;
            //let falloff_factor = light_surface_dot / (light.falloff.0 + light.falloff.1*light_distance + light.falloff.2*light_distance*light_distance);
            //total_color += phong_color * light.color_intensity() * light_through * falloff_factor;
            total_color += light_through * phong_color * light_surface_dot * light.get_intensity_at_distance(light_distance);
        }

        total_color
    }
}

#[derive(Clone)]
pub struct ReflectionShader {
    reflectivity: Color,
}

impl ReflectionShader {
    pub fn new(reflectivity: Color) -> ReflectionShader {
        ReflectionShader{reflectivity}
    }
}

impl Shadable for ReflectionShader {
    fn get_color(&self, scene: &Scene, ray: Ray, hit_point: DVec3, surface_normal: DVec3) -> Color {
        let reflected_ray = ray.reflect_off(hit_point, surface_normal);
        self.reflectivity * scene.cast_ray(reflected_ray.contributes(self.reflectivity))
    }
}

#[derive(Clone)]
pub struct TranslucentShader {
    // TODO: separate translucency and color
    translucency: Color,
    refractive_index: f64,
}

impl TranslucentShader {
    pub fn new(translucency: Color, refractive_index: f64) -> TranslucentShader {
        TranslucentShader{translucency, refractive_index}
    }
}

impl Shadable for TranslucentShader {
    fn get_color(&self, scene: &Scene, ray: Ray, hit_point: DVec3, surface_normal: DVec3) -> Color {
        let transmitted_ray = ray.transmit_through(hit_point, surface_normal, self.refractive_index);
        let color = scene.cast_ray(transmitted_ray.contributes(self.translucency));
        if surface_normal.dot(transmitted_ray.direction) < 0.0 {
            self.translucency * color
        }
        else {
            color
        }
    }

    fn get_opacity(&self) -> Color {
        Color::WHITE - self.translucency
    }
}