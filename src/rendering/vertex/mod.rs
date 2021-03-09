pub mod vertex_data;

use crate::rendering::vertex::vertex_data::f32_f32_f32;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    #[location = "0"]
    pub pos: f32_f32_f32,
    #[location = "1"]
    pub color: f32_f32_f32,
}