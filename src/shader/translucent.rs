use super::*;
use geometry::Ray;
use euler::DVec3;
use color::*;

#[derive(Clone)]
pub struct TranslucentShader {
    // TODO: define as absorption of light passing through. Probably use in medium instead of surface
    translucency: Color,
    refractive_index: f64,
}

impl TranslucentShader {
    pub fn new(translucency: Color, refractive_index: f64) -> Box<TranslucentShader> {
        Box::new(TranslucentShader{translucency, refractive_index})
    }

    // Returns ratio of light (transmitted, reflected)
    fn get_fresnel(&self, intersect: &Intersect) -> (f64, f64) {
        use std::mem::swap;

        let mut cosi = intersect.ray.direction.dot(intersect.surface_normal);
        let mut etai = 1.0;
        let mut etat = self.refractive_index;

        if cosi > 0.0 {
            swap(&mut etai, &mut etat);
        }

        let sint = etai / etat * (1.0 - cosi*cosi).max(0.0).sqrt();

        if sint >= 1.0 {
            (0.0, 1.0)
        }
        else {
            let cost = (1.0 - sint*sint).max(0.0).sqrt();
            cosi = cosi.abs();

            let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
            let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));

            let reflectance = (rs * rs + rp * rp) * 0.5;
            assert!(reflectance <= 1.0);
            (1.0 - reflectance, reflectance)
        }
    }
}

impl Shadable for TranslucentShader {
    fn get_color(&self, scene: &Scene, intersect: Intersect) -> Color {

        let (kt, kr) = self.get_fresnel(&intersect);
        let reflected_ray = intersect.ray.reflect_off(intersect.hit_point, intersect.surface_normal);
        let transmitted_ray = intersect.ray.transmit_through(intersect.hit_point, intersect.surface_normal, self.refractive_index);

        // TODO: make TranslucentShader into TransparentShader, remove self.translucency
        let color_t = if kt > 0.0 { scene.cast_ray(transmitted_ray.contributes(Color::WHITE * kt)) } else { Color::BLACK };
        let color_r = if kr > 0.0 { scene.cast_ray(reflected_ray.contributes(Color::WHITE * kr)) } else { Color::BLACK };
        //scene.cast_ray(transmitted_ray.contributes(self.translucency))

        (kr * color_r) + (kt * color_t)
    }

    fn get_opacity(&self) -> Color {
        Color::WHITE - self.translucency
    }
}