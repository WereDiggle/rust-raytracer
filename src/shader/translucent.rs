use super::*;

#[derive(Clone)]
pub struct TranslucentShader {
    // TODO: separate translucency and color
    translucency: Color,
    refractive_index: f64,
}

impl TranslucentShader {
    pub fn new(translucency: Color, refractive_index: f64) -> Box<TranslucentShader> {
        Box::new(TranslucentShader{translucency, refractive_index})
    }
}

impl Shadable for TranslucentShader {
    fn get_color(&self, scene: &Scene, intersect: Intersect) -> Color {
        let transmitted_ray = intersect.ray.transmit_through(intersect.hit_point, intersect.surface_normal, self.refractive_index);
        let color = scene.cast_ray(transmitted_ray.contributes(self.translucency));
        if intersect.surface_normal.dot(transmitted_ray.direction) < 0.0 {
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