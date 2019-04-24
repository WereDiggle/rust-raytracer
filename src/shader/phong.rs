use super::*;

#[derive(Clone)]
pub struct PhongShader {
    diffuse: Color,
    specular: Color,
    ambient: Color,
    shininess: f64,
}

impl PhongShader {
    pub fn new(diffuse: Color, specular: Color, ambient: Color, shininess: f64) -> Box<PhongShader> {
        Box::new(PhongShader{diffuse, specular, ambient, shininess})
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
    fn get_color(&self, scene: &Scene, intersect: Intersect) -> Color {
        assert!(intersect.surface_normal.length() - 1.0 < 0.0001, "normal: {}", intersect.surface_normal);
        let mut total_color = self.ambient * scene.ambient_light.color_intensity();
        for light in &scene.lights {

            let illums = light.get_illums_at(scene, intersect);
            for illum in illums.iter() {
                if let Illum::Lit{surface_dot, light_direction, intensity} = illum {
                    let reflection_direction = (2.0*surface_dot*intersect.surface_normal - *light_direction).normalize();
                    let specular_factor = reflection_direction.dot(-1.0*intersect.ray.direction).max(0.0).powf(self.shininess) / surface_dot;
                    let phong_color = self.specular * specular_factor + self.diffuse;
                    total_color += phong_color * *surface_dot * *intensity;
                }
            }
        }

        total_color
    }
}