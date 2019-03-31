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
            let light_direction = -1.0 * light.get_direction_to(intersect.hit_point);
            let light_surface_dot = light_direction.dot(intersect.surface_normal);
            if light_surface_dot <= 0.0 {
                continue;
            }

            let shadow_ray = Ray::new(intersect.hit_point, light_direction, 1);
            let light_distance = light.get_distance_to(intersect.hit_point);

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

            let reflection_direction = (2.0*light_surface_dot*intersect.surface_normal - light_direction).normalize();
            let specular_factor = reflection_direction.dot(-1.0*intersect.ray.direction).max(0.0).powf(self.shininess) / light_surface_dot;
            let phong_color = self.specular * specular_factor + self.diffuse;
            //let falloff_factor = light_surface_dot / (light.falloff.0 + light.falloff.1*light_distance + light.falloff.2*light_distance*light_distance);
            //total_color += phong_color * light.color_intensity() * light_through * falloff_factor;
            total_color += light_through * phong_color * light_surface_dot * light.get_intensity_at_distance(light_distance);
        }

        total_color
    }
}