pub mod vertex_data;

use crate::rendering::vertex::vertex_data::{f32_f32_f32, f32_f32, u2_u10_u10_u10_rev_float};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    #[location = "0"]
    pub pos: f32_f32_f32,
    #[location = "1"]
    pub color: u2_u10_u10_u10_rev_float,
    #[location = "2"]
    pub uv: f32_f32,
}
