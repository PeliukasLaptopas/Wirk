use nalgebra::Vector2;
use crate::rendering::vertex::vertex_data::u2_u10_u10_u10_rev_float;
use crate::rendering::vertex::Vertex;
use crate::rendering::camera_2d::Camera2D;
use crate::rendering::shader::program::Program;
use gl::Gl;
use crate::rendering::shader::buffer;
use std::ffi::CString;


struct UIGlyph {
    depth: f32,

    top_left:     Vertex,
    bottom_left:  Vertex,
    top_right:    Vertex,
    bottom_right: Vertex,
}

pub struct UIBatch {
    glyphs: Vec<UIGlyph>,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl UIBatch {
    pub fn new(gl: &Gl) -> UIBatch {
        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        UIBatch {
            glyphs: vec![],
            vbo,
            vao,
        }
    }

    pub fn create_render_batches(&self) {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(self.glyphs.len() * 6);
        unsafe { vertices.set_len(self.glyphs.len() * 6); } //todo NO!

        if (self.glyphs.is_empty()) {
            return; //no batches to create
        }

        let mut cv: usize = 0; //current vertex

        for cg in (1..self.glyphs.len()) { //cg - current glyph
            vertices[cv] = self.glyphs[cg].top_left; cv += 1;
            vertices[cv] = self.glyphs[cg].bottom_left; cv += 1;
            vertices[cv] = self.glyphs[cg].bottom_right; cv += 1;
            vertices[cv] = self.glyphs[cg].bottom_right; cv += 1;
            vertices[cv] = self.glyphs[cg].top_right; cv += 1;
            vertices[cv] = self.glyphs[cg].top_left; cv += 1;
        }

        self.vbo.bind();
        self.vbo.dynamic_draw_data(&vertices); //todo use orphaned buffer
        self.vbo.unbind();
    }

    pub fn add_to_batch(&mut self, pos: Vector2<f32>, scale: Vector2<f32>, color: u2_u10_u10_u10_rev_float, depth: f32) {
        let new_glyph = UIGlyph {
            depth,
            top_left: Vertex {
                pos: (pos.x - scale.x / 2.0, pos.y - scale.y / 2.0).into(),
                color,
                uv: (0.0, 1.0).into()
            },
            bottom_left: Vertex {
                pos: (pos.x - scale.x / 2.0, pos.y - scale.y / 2.0).into(),
                color,
                uv: (0.0, 0.0).into()
            },
            top_right: Vertex {
                pos: (pos.x - scale.x / 2.0, pos.y - scale.y / 2.0).into(),
                color,
                uv: (1.0, 1.0).into()
            },
            bottom_right: Vertex {
                pos: (pos.x - scale.x / 2.0, pos.y - scale.y / 2.0).into(),
                color,
                uv: (1.0, 0.0).into()
            }
        };

        self.glyphs.push(new_glyph);
    }

    pub fn begin(&mut self) {
        self.glyphs.clear();
    }

    pub fn end(&mut self) {
        self.glyphs.sort_by(|g1, g2| g1.depth.partial_cmp(&g2.depth).unwrap()); //stable sort
        self.create_render_batches();
    }

    pub fn render_batch(&mut self, camera: &mut Camera2D, program: &mut Program, gl: &Gl) {
        program.use_program();
        self.vao.bind();

        unsafe {
            let loc = gl.GetUniformLocation(program.id, CString::new("P").unwrap().as_ptr());
            gl.UniformMatrix4fv(
                loc,
                1,
                gl::FALSE,
                camera.camera_matrix.as_slice().as_ptr() as *const f32
            );


            for i in 0..(self.glyphs.len() as i32) {
                gl.DrawArrays(
                    gl::TRIANGLES,
                    i * 6,
                    6
                );
            }

            for glyph in &self.glyphs {

            }
        }
    }
}