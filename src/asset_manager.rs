use texture::*;
use normal_map::*;
use image::RgbImage;
use std::sync::Arc;

use std::collections::HashMap;

// TODO: Should all the path related functions be in here?
// And all the actual constructors use their respect asset, ie RgbImage

// Factory for creating assets that benefit from being shared.
// Anything that involves reading in a large file
pub struct AssetManager<'a> {
    texture_cache: HashMap<&'a str, Box<TextureMappable + Send + Sync>>,
    normal_map_cache: HashMap<&'a str, Box<NormalMappable + Send + Sync>>,
}

// Methods for creating assets
impl<'a> AssetManager<'a> {
    pub fn new() -> AssetManager<'a> {
        AssetManager {
            texture_cache: HashMap::new(),
            normal_map_cache: HashMap::new(),
        }
    }

    pub fn image_texture_from_path(&self, path: &str) -> Box<TextureMappable> {
        // Check if cache already has asset
        if let Some(texture) = self.texture_cache.get(path) {
            texture.clone()
        }
        else {
            // TODO: better error handling
            ImageTexture::new(image::open(path).unwrap().to_rgb())
        }

    }

    pub fn bump_map_from_path(&self, path: &str, depth: f64) -> Box<NormalMappable> {
        if let Some(bump_map) = self.normal_map_cache.get(path) {
            bump_map.clone()
        }
        else {
            BumpMap::new(image::open(path).unwrap().to_luma(), depth)
        }
    }

    pub fn normal_map_from_path(&self, path: &str) -> Box<NormalMappable> {
        // Check if cache already has asset
        if let Some(normal_map) = self.normal_map_cache.get(path) {
            normal_map.clone()
        }
        else {
            // TODO: better error handling
            NormalMap::new(image::open(path).unwrap().to_rgb())
        }
    }
}