use gl::types::GLuint;
use crate::rendering::shader::buffer;
use crate::rendering::vertex::Vertex;
use nalgebra::{Vector4, Vector2};
use crate::rendering::vertex::vertex_data::{u2_u10_u10_u10_rev_float, f32_f32};
use gl::Gl;
use sdl2::log::Category::Render;
use std::ffi::CString;
use crate::rendering::shader::program::Program;
use crate::rendering::camera_2d::Camera2D;


struct Glyph {
    texture: GLuint,
    depth: f32,

    top_left:     Vertex,
    bottom_left:  Vertex,
    top_right:    Vertex,
    bottom_right: Vertex,
}

struct RenderBatch {
    offset: GLuint,
    vertices_count: GLuint,
    texture: GLuint,
}

pub struct SpriteBatch {
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    glyphs: Vec<Glyph>,
    render_batches: Vec<RenderBatch>,
}

impl SpriteBatch {
    pub fn new(gl: &Gl) -> SpriteBatch {
        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        SpriteBatch {
            vbo,
            vao,
            glyphs: vec![],
            render_batches: vec![]
        }
    }

    fn create_render_batches(&mut self) {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(self.glyphs.len() * 6);
        unsafe { vertices.set_len(self.glyphs.len() * 6); } //todo NO!

        if (self.glyphs.is_empty()) {
            return; //no batches to create
        }

        let mut offset = 0;
        let render_batch = RenderBatch {
            offset,
            vertices_count: 6,
            texture: self.glyphs[0].texture
        };

        self.render_batches.push(render_batch); //emplace_back ??

        let mut cv: usize = 0; //current vertex

        // println!("Size of glyps {}", self.glyphs.len());
        // println!("Size of vertices {}", vertices.len());

        vertices[cv] = self.glyphs[0].top_left; cv += 1;
        vertices[cv] = self.glyphs[0].bottom_left; cv += 1;
        vertices[cv] = self.glyphs[0].bottom_right; cv += 1;
        vertices[cv] = self.glyphs[0].bottom_right; cv += 1;
        vertices[cv] = self.glyphs[0].top_right; cv += 1;
        vertices[cv] = self.glyphs[0].top_left; cv += 1;
        offset += 6;

        for cg in (1..self.glyphs.len()) { //cg - current glyph
            let render_batch = RenderBatch {
                offset,
                vertices_count: 6,
                texture: self.glyphs[cg].texture
            };
            if (self.glyphs[cg].texture != self.glyphs[cg - 1].texture) {
                self.render_batches.push(render_batch); //emplace_back ??
            } else {
                self.render_batches.last_mut().unwrap().vertices_count += 6;
            }


            vertices[cv] = self.glyphs[cg].top_left; cv += 1;
            vertices[cv] = self.glyphs[cg].bottom_left; cv += 1;
            vertices[cv] = self.glyphs[cg].bottom_right; cv += 1;
            vertices[cv] = self.glyphs[cg].bottom_right; cv += 1;
            vertices[cv] = self.glyphs[cg].top_right; cv += 1;
            vertices[cv] = self.glyphs[cg].top_left; cv += 1;

            offset += 6;
        }

        self.vbo.bind();
        self.vbo.dynamic_draw_data(&vertices); //todo use orphaned buffer
        self.vbo.unbind();
    }

    pub fn init(&mut self) {
    }

    pub fn begin(&mut self) {
        self.render_batches.clear();
        self.glyphs.clear();
    }

    pub fn end(&mut self) {
        self.glyphs.sort_by(|g1, g2| g1.texture.partial_cmp(&g2.texture).unwrap()); //stable sort
        self.create_render_batches();
    }

    /*
    B------C
    |      |
    |      |
    A------D

    A - our pivot (start)
    D = vec2(A.x + width, A.y)
    B = vec2(A.x, A.y + height)
    C = vec2(A.x + width, A.y + height)
    */
    pub fn add_to_batch( //todo fix from vec4's to vec2's position and width
            &mut self,
            dest_rect: Vector4<f32>,
            uv_rect: Vector4<f32>,
            color: u2_u10_u10_u10_rev_float,
            texture: GLuint,
            depth: f32,
        ) {
        let new_glyph = Glyph {
            texture,
            depth,
            top_left: Vertex {
                pos: (dest_rect.x, dest_rect.y + dest_rect.w).into(),
                color,
                uv: (uv_rect.x, uv_rect.y + uv_rect.w).into()
            },
            bottom_left: Vertex {
                pos: (dest_rect.x, dest_rect.y).into(),
                color,
                uv: (uv_rect.x, uv_rect.y).into()
            },
            top_right:  Vertex {
                pos: (dest_rect.x + dest_rect.z, dest_rect.y + dest_rect.w).into(),
                color,
                uv: (uv_rect.x + uv_rect.z, uv_rect.y + uv_rect.w).into()
        },
            bottom_right: Vertex {
                pos: (dest_rect.x + dest_rect.z, dest_rect.y).into(),
                color,
                uv: (uv_rect.x + uv_rect.z, uv_rect.y).into()
            }
        };

        self.glyphs.push(new_glyph);
    }

    pub fn render_batch(&mut self, time: &f32, camera: &mut Camera2D, program: &mut Program, gl: &Gl) {
        program.use_program();
        self.vao.bind();

        unsafe {
            gl.ActiveTexture(gl::TEXTURE0); //todo is this expensive in this function ??

            let loc = gl.GetUniformLocation(program.id, CString::new("mySampler").unwrap().as_ptr());
            gl.Uniform1i(loc, 0);

            let loc = gl.GetUniformLocation(program.id, CString::new("time").unwrap().as_ptr());
            gl.Uniform1f(loc, *time);
            let loc = gl.GetUniformLocation(program.id, CString::new("P").unwrap().as_ptr());
            gl.UniformMatrix4fv(
                loc,
                1,
                gl::FALSE,
                camera.camera_matrix.as_slice().as_ptr() as *const f32
            );

            for batch in &self.render_batches {
                gl.BindTexture(gl::TEXTURE_2D, batch.texture);

                gl.DrawArrays(
                    gl::TRIANGLES,
                    batch.offset as i32,
                    batch.vertices_count as i32
                );
            }
        }
    }
}