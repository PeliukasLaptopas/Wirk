pub mod texture_cache;
pub mod resource_utils;

use crate::resources::resource_utils::resource_name_to_path;
use crate::resources::errors::ResourceError::*;
use crate::resources::errors::ResourceError;
use crate::resources::texture_cache::TextureCache;
use std::path::{PathBuf, Path};
use std::ffi::CString;
use std::{fs};
use std::io::Read;
use image::io::Reader as ImageReader;
use image::DynamicImage::*;
use std::collections::BTreeMap;
use image::GenericImageView;
use crate::rendering::texture::Texture;
use sdl2::pixels::Color;

pub mod errors;

pub struct Resources<'a> {
    pub root_path: PathBuf, //todo not pub
    texture_cache: BTreeMap<&'a str, Texture> //todo use Gltexture class (create later)
}

impl Resources<'_> {
    pub fn get_texture(&mut self, name: &'static str, gl: &gl::Gl) -> Result<Texture, failure::Error> {
        let texture_opt = self.texture_cache.get(name);

        return match texture_opt {
            Some(texture) => Ok(Texture {
                id: texture.id,
                width: texture.width,
                height: texture.height,
            }),
            None => {
                let texture_id = Resources::load_png(name, self, gl)?;

                self.texture_cache.insert(name.clone(), Texture {
                    id: texture_id.id,
                    width: texture_id.width,
                    height: texture_id.height
                });

                Ok(texture_id)
            }
        }
    }

    pub fn load_png(name: &str, resources: &Resources, gl: &gl::Gl) -> Result<Texture, failure::Error> {
        let img = ImageReader::open(resources.root_path.join(name).into_os_string().into_string().unwrap())?.decode()?; //ImageRgba8

        let width = img.dimensions().0;
        let height = img.dimensions().1;

        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut bytes, image::ImageOutputFormat::Png)?;

        let mut texture_id: gl::types::GLuint = 0;

        unsafe {
            gl.GenTextures(1, &mut texture_id);

            gl.BindTexture(gl::TEXTURE_2D, texture_id);
            gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, height as i32, 0, gl::RGBA as u32, gl::UNSIGNED_BYTE, img.as_bytes().as_ptr() as *const std::os::raw::c_void);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

            gl.GenerateMipmap(gl::TEXTURE_2D);

            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        //todo clear bytes?
        Ok(Texture {
            id: texture_id,
            width: width,
            height: height,
        }) //todo check if error
    }

    pub fn generate_from_text(&self, text: String, gl: &gl::Gl) -> Result<Texture, failure::Error> {
        let mut texture_id: gl::types::GLuint = 0;

        let ttf_context = sdl2::ttf::init().map_err(|e| format_err!("{:?}", e))?;

        // Load a font
        let mut font = ttf_context.load_font(self.root_path.join("font.otf"), 128).map_err(|e| format_err!("{:?}", e))?;

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(text.as_str())
            .blended(Color::RGBA(35, 121, 100, 255))
            .map_err(|e| format_err!("{:?}", e))?;

        unsafe {
            let pixels = (*surface.raw()).pixels as *const std::os::raw::c_void;

            gl.GenTextures(1, &mut texture_id);

            gl.BindTexture(gl::TEXTURE_2D, texture_id);
            gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, surface.width() as i32, surface.height() as i32, 0, gl::RGBA as u32, gl::UNSIGNED_BYTE, pixels);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

            gl.GenerateMipmap(gl::TEXTURE_2D);

            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        //todo clear bytes?
        Ok(Texture {
            id: texture_id,
            width: surface.width(),
            height: surface.height(),
        }) //todo check if error
    }

    pub fn from_relative_path(rel_path: &Path) -> Result<Resources, ResourceError> {

        let exe_file_name = ::std::env::current_exe()
            .map_err(|_| FailedToGetExePath)?;

        let exe_path = exe_file_name.parent()
            .ok_or(FailedToGetExePath)?;

        let new_tree: BTreeMap<&str, Texture> = BTreeMap::new();

        Ok(Resources {
            root_path: exe_path.join(rel_path),
            texture_cache: new_tree,
        })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<CString, ResourceError> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path,resource_name)
        )?;

        // allocate buffer of the same size as file
        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut buffer)?;

        // check for nul byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(FileContainsNil);
        }

        Ok(unsafe { CString::from_vec_unchecked(buffer) })
    }
}