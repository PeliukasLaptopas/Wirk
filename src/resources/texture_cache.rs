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
    }/*

    pub fn get_texture(&mut self, name: &'a str, resources: &Resources, gl: &gl::Gl) -> Result<gl::types::GLuint, failure::Error> { //todo make to be result later
        let texture_opt = self.textures.get(name);

        println!("TESTING {}", self.textures.is_empty());
        for (movie, review) in &self.textures {
            println!("{}: \"{}\"", movie, review);
        }

        match texture_opt {
            Some(texture) => {
                println!("BUVO JAU");
                return Ok(*texture)
            },

            None =>  {
                println!("NAUJAS TEXTURE");

                let texture_id = Resources::load_png(name, resources, gl)?;


                self.textures.insert(name.clone(), texture_id);
                return Ok(texture_id);
            }
        }
    }*/
}
