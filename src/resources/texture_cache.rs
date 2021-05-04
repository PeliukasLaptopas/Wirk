use std::collections::BTreeMap;
use std::ffi::CString;
use image::io::Reader as ImageReader;
use image::DynamicImage::*;
use crate::resources::Resources;

//todo ONLY TEXTURES should be immutable. get_texture() uses mut self so all values are mutable
pub struct TextureCache<'a> {
    textures: BTreeMap<&'a str, gl::types::GLuint> //todo use Gltexture class (create later)
}

impl<'a> TextureCache<'a> {
    pub fn new() -> TextureCache<'a> {
        let new_tree: BTreeMap<&str, gl::types::GLuint> = BTreeMap::new();

        TextureCache {
            textures: new_tree,
        }
    }
}
