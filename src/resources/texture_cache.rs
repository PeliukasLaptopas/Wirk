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

    pub fn get_texture(&mut self, name: &'a str, resources: &Resources, gl: &gl::Gl) -> Result<gl::types::GLuint, failure::Error> { //todo make to be result later
        //lookup the texture to see if i have it in the map
        let texture_opt = self.textures.get(name);

        match texture_opt {
            Some(texture) => {
                println!("BUVO JAU");
                return Ok(*texture)
            },

            None =>  {
                println!("NAUJAS TEXTURE");

                let img = ImageReader::open(resources.root_path.join(name).into_os_string().into_string().unwrap())?.decode()?; //ImageRgba8

                let mut bytes: Vec<u8> = Vec::new();
                img.write_to(&mut bytes, image::ImageOutputFormat::Png)?;

                let mut texture_id: gl::types::GLuint = 0;
                unsafe {
                    gl.GenTextures(1, &mut texture_id);
                    gl.BindTexture(gl::TEXTURE_2D, texture_id);
                    gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, 64, 64, 0, gl::RGBA as u32, gl::UNSIGNED_BYTE, img.into_bytes().as_ptr() as *const std::os::raw::c_void);
                    gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
                    gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                    gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                    gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);

                    gl.GenerateMipmap(gl::TEXTURE_2D);

                    gl.BindTexture(gl::TEXTURE_2D, 0);
                }

                self.textures.insert(name.clone(), texture_id);
                return Ok(texture_id);
            }
        }
    }
}
