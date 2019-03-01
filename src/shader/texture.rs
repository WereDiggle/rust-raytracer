use super::*;

#[derive(Clone)]
pub struct TextureShader {
    texture: Box<TextureMappable + Send + Sync>,
}

impl TextureShader {
    pub fn new(texture: Box<TextureMappable + Send + Sync>) -> Box<TextureShader> {
        Box::new(TextureShader {
            texture,
        })
    } 
}

impl Shadable for TextureShader {
    fn get_color(&self, _: &Scene, intersect: Intersect) -> Color {
        self.texture.get_color(intersect.surface_coord)
    }
}