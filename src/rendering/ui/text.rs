use nalgebra::Vector2;
use crate::rendering::vertex::Vertex;
use crate::rendering::vertex::vertex_data::{f32_f32, u2_u10_u10_u10_rev_float};
use crate::rendering::shader::buffer;
use crate::rendering::shader::program::Program;
use crate::rendering::sprite::rigid_body_2d::RigidBody2D;
use crate::rendering::ui::ui_batch::UIBatch;
use crate::rendering::sprite::sprite_batch::SpriteBatch;
use crate::rendering::texture::Texture;

pub struct Text {
    // text: String,
    pos: Vector2<f32>,
    scale: Vector2<f32>,
    color: u2_u10_u10_u10_rev_float,
    texture_id: gl::types::GLuint,
}

impl Text {
    pub fn new(pos: Vector2<f32>, scale: Vector2<f32>, color: u2_u10_u10_u10_rev_float, texture: &Texture) -> Text {
        Text {
            pos,
            scale,
            color,
            texture_id: texture.id
        }
    }

    pub fn draw(&self, sprite_batch: &mut SpriteBatch, angle: f32/*, angle: f32*/) {
        sprite_batch.add_to_batch(
            Vector2::new(self.pos.x, self.pos.y),
            Vector2::new(self.scale.x, self.scale.y),
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 1.0),
            self.color,
            self.texture_id,
            &angle,
            0.0
        );
    }
}
