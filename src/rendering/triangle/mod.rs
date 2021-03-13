use crate::rendering::shader::buffer;
use crate::resources::Resources;
use crate::rendering::vertex::Vertex;
use crate::rendering::shader::program::Program;
extern crate nalgebra;
use nalgebra::*;

pub struct Triangle {
    program: Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(pos: &Vector2<f32>, scale: &Vector2<f32>, res: &Resources, gl: &gl::Gl) -> Result<Triangle, failure::Error> {

        // set up shader program
        let program = Program::from_res(gl, res, "shaders/triangle")?;

        // set up vertex buffer object
        let vertices: Vec<Vertex> = vec![
            //First triangle:
            Vertex { //top right
                pos: (pos.x + scale.x, pos.y + scale.y, 0.0).into(),
                color: (1.0, 0.0, 0.0, 1.0).into()
            },
            Vertex { // top left
                pos: (pos.x, pos.y + scale.y, 0.0).into(),
                color: (0.0, 1.0, 0.0, 1.0).into()
            },
            Vertex { //bottom left
                pos: (pos.x, pos.y, 0.0).into(),
                color: (0.0, 0.0, 1.0, 1.0).into()
            },

            //second triangle:
            Vertex { //bottom left
                pos: (pos.x, pos.y, 0.0).into(),
                color: (1.0, 0.0, 0.0, 1.0).into()
            },
            Vertex { // bottom right
                pos: (pos.x + scale.x, pos.y, 0.0).into(),
                color: (0.0, 1.0, 0.0, 1.0).into()
            },
            Vertex { //top right
                pos: (pos.x + scale.x, pos.y + scale.y, 0.0).into(),
                color: (0.0, 0.0, 1.0, 1.0).into()
            }
        ];

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

        Ok(Triangle {
            program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.use_program();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                6 // number of indices to be rendered
            );
        }
    }
}