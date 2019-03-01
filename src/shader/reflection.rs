use super::*;

#[derive(Clone)]
pub struct ReflectionShader {
    reflectivity: Color,
}

impl ReflectionShader {
    pub fn new(reflectivity: Color) -> Box<ReflectionShader> {
        Box::new(ReflectionShader{reflectivity})
    }
}

impl Shadable for ReflectionShader {
    fn get_color(&self, scene: &Scene, intersect: Intersect) -> Color {
        let reflected_ray = intersect.ray.reflect_off(intersect.hit_point, intersect.surface_normal);
        self.reflectivity * scene.cast_ray(reflected_ray.contributes(self.reflectivity))
    }
}