pub mod sprite_batch;

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
use gl::types::GLuint;
use crate::rendering::camera_2d::Camera2D;

pub struct Sprite {
    pub program: Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
    texture_id: GLuint,
}

impl Sprite {
    pub fn new(
        pos: &Vector2<f32>,
        scale: &Vector2<f32>,
        texture_name: &'static str,
        res: &mut Resources<'static>,
        gl: &gl::Gl
    ) -> Result<Sprite, failure::Error> {

        // set up shader program
        let program = Program::from_res(gl, res, "shaders/triangle")?;

        // set up vertex buffer object
        let vertices: Vec<Vertex> = vec![
            //First triangle:
            Vertex { //top right
                pos: (pos.x + scale.x, pos.y + scale.y).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (1.0, 1.0).into()
            },
            Vertex { // top left
                pos: (pos.x, pos.y + scale.y).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (0.0, 1.0).into()
            },
            Vertex { //bottom left
                pos: (pos.x, pos.y).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (0.0, 0.0).into()
            },

            //second triangle:
            Vertex { //bottom left
                pos: (pos.x, pos.y).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (0.0, 0.0).into()
            },
            Vertex { // bottom right
                pos: (pos.x + scale.x, pos.y).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (1.0, 0.0).into()
            },
            Vertex { //top right
                pos: (pos.x + scale.x, pos.y + scale.y).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                uv: (1.0, 1.0).into()
            }
        ];


        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);

        // set up vertex array object
        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        Vertex::vertex_attrib_pointers(gl); //vbo here is bind
        vbo.unbind();
        vao.unbind();


        let texture_id = res.get_texture(texture_name, gl)?; //todo should get width and height from this function and store that here in sprite

        Ok(Sprite {
            program,
            _vbo: vbo,
            vao,
            texture_id
        })
    }

    pub fn draw(&self, camera: &mut Camera2D, gl: &gl::Gl, time: &f32) {
        self.program.use_program();
        self.vao.bind();

        unsafe {
            let loc = gl.GetUniformLocation(self.program.id, CString::new("mySampler").unwrap().as_ptr());
            gl.Uniform1i(loc, 0);

            let loc = gl.GetUniformLocation(self.program.id, CString::new("time").unwrap().as_ptr());
            gl.Uniform1f(loc, *time);

            let loc = gl.GetUniformLocation(self.program.id, CString::new("P").unwrap().as_ptr());
            gl.UniformMatrix4fv(
                loc,
                1,
                gl::FALSE,
                camera.ortho_matrix.as_slice().as_ptr() as *const f32
            );

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