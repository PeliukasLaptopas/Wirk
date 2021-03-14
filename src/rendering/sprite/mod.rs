use crate::rendering::shader::buffer;
use crate::resources::Resources;
use crate::rendering::vertex::Vertex;
use crate::rendering::shader::program::Program;
use crate::resources::texture_cache::*;
extern crate nalgebra;
use nalgebra::*;
use std::ffi::CString;
use image::io::Reader as ImageReader;
use image::DynamicImage::*;

pub struct Sprite {
    pub program: Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
    texture_id: gl::types::GLuint,
}

impl Sprite {
    pub fn new(pos: &Vector2<f32>, scale: &Vector2<f32>, res: &Resources, gl: &gl::Gl) -> Result<Sprite, failure::Error> {

        // set up shader program
        let program = Program::from_res(gl, res, "shaders/triangle")?;

        // set up vertex buffer object
        let vertices: Vec<Vertex> = vec![
            //First triangle:
            Vertex { //top right
                pos: (pos.x + scale.x, pos.y + scale.y, 0.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (1.0, 1.0).into()
            },
            Vertex { // top left
                pos: (pos.x, pos.y + scale.y, 0.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (0.0, 1.0).into()
            },
            Vertex { //bottom left
                pos: (pos.x, pos.y, 0.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (0.0, 0.0).into()
            },

            //second triangle:
            Vertex { //bottom left
                pos: (pos.x, pos.y, 0.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (0.0, 0.0).into()
            },
            Vertex { // bottom right
                pos: (pos.x + scale.x, pos.y, 0.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (1.0, 0.0).into()
            },
            Vertex { //top right
                pos: (pos.x + scale.x, pos.y + scale.y, 0.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (1.0, 1.0).into()
            }
        ];

        /*let img = ImageReader::open(res.root_path.join("water.png").into_os_string().into_string().unwrap())?.decode()?; //ImageRgba8

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
        }*/

        //todo should be in main
        let mut textureCache = TextureCache::new();
        let texture = textureCache.get_texture("water.png", res, gl)?;


        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        // set up vertex array object

        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Sprite {
            program,
            _vbo: vbo,
            vao,
            texture_id: texture //todo fix
        })
    }

    pub fn draw(&self, gl: &gl::Gl, time: &f32) {
        self.program.use_program();
        self.vao.bind();

        unsafe {
            let loc = gl.GetUniformLocation(self.program.id, CString::new("mySampler").unwrap().as_ptr());
            gl.Uniform1i(loc, 0);

            let loc = gl.GetUniformLocation(self.program.id, CString::new("time").unwrap().as_ptr());
            gl.Uniform1f(loc, *time);


            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, self.texture_id);

            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                6 // number of indices to be rendered
            );
        }
    }
}