use scene::Scene;
use scene::Transformable;
use color::Color;
use euler::DVec3;
use geometry::Ray;

pub trait Shadable: ShadableClone {
    fn get_color(&self, scene: &Scene, ray: Ray, hit_point: DVec3, surface_normal: DVec3) -> Color;
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

impl Shadable for PhongShader {
    fn get_color(&self, scene: &Scene, ray: Ray, hit_point: DVec3, surface_normal: DVec3) -> Color {
        assert!(surface_normal.length() - 1.0 < 0.000001);
        let mut total_color = self.ambient * scene.ambient_light.color_intensity();
        for light in &scene.lights {
            let light_direction = (light.position - hit_point).normalize();
            let light_surface_dot = light_direction.dot(surface_normal);
            if light_surface_dot <= 0.0 {
                continue;
            }

            let light_ray = Ray::new(hit_point, light_direction);
            let light_distance = (light.position - hit_point).length();

            if let Some(_) = scene.root.partial_trace(light_ray, light_distance) {
                continue;
            }

            let reflection_direction = (2.0*light_surface_dot*surface_normal - light_direction).normalize();
            let specular_factor = reflection_direction.dot(-1.0*ray.direction).max(0.0).powf(self.shininess) / light_surface_dot;
            let phong_color = self.specular * specular_factor + self.diffuse;
            let falloff_factor = light_surface_dot / (light.falloff.0 + light.falloff.1*light_distance + light.falloff.2*light_distance*light_distance);
            total_color += phong_color * light.color_intensity() * falloff_factor;
        }

        total_color
    }
}