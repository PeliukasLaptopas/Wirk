pub mod sprite_batch;
pub mod rigid_body_2d;

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
use crate::rendering::sprite::sprite_batch::SpriteBatch;
use wrapped2d::b2::{BodyHandle, World, BodyType};
use wrapped2d::user_data::NoUserData;
use wrapped2d::b2;
use nalgebra::{Vector4, Vector2};
use wrapped2d::wrap::Wrapped;
use crate::rendering::vertex::vertex_data::u2_u10_u10_u10_rev_float;
use crate::rendering::texture::Texture;
use crate::rendering::sprite::rigid_body_2d::{RigidBody2D, ColliderType};
use wrapped2d::common::math::Vec2;
use wrapped2d::dynamics::body::BodyType::Dynamic;

pub struct Sprite {
    pub texture_id: GLuint,
    pub rigid_body_2d: RigidBody2D,
    pub color: u2_u10_u10_u10_rev_float,
}

impl Sprite {
    pub fn new(
        pos: Vector2<f32>,
        texture_name: &'static str,
        body_type: &BodyType,
        collider_type: ColliderType,
        color: u2_u10_u10_u10_rev_float,
        world: &mut World<NoUserData>,
        res: &mut Resources<'static>,
        gl: &gl::Gl,
    ) -> Result<Sprite, failure::Error> {
        let texture = res.get_texture(texture_name, gl)?; //todo should get width and height from this function and store that here in sprite

        let rigid_body_2d = match collider_type {
            ColliderType::Box(scale) => RigidBody2D::new_box_body(world, body_type, Vec2 {x:pos.x, y:pos.y}, scale),
            ColliderType::Circle(radius) => RigidBody2D::new_circle_body(world, body_type, Vec2 {x:pos.x, y:pos.y}, radius)
        };

        Ok(Sprite {
            texture_id: texture.id,
            rigid_body_2d,
            color,
        })
    }

    pub fn get_pos(&self, world: &World<NoUserData>) -> Vector2<f32> {
        Vector2::new(world.body(self.rigid_body_2d.body).position().x, world.body(self.rigid_body_2d.body).position().y)
    }

    pub fn draw(&self, world: &mut World<NoUserData>, camera: &mut Camera2D, gl: &gl::Gl, sprite_batch: &mut SpriteBatch, angle: f32) {
        let b2_body = world.body_mut(self.rigid_body_2d.body);
        let pos = Vector2::new(b2_body.position().x, b2_body.position().y);

        let scale = match self.rigid_body_2d.collider_type {
            ColliderType::Circle(radius) => { Vector2::new(radius, radius) },
            ColliderType::Box(scale) => { Vector2::new(scale.x, scale.y) }
        };
        // let pos = match self.rigid_body_2d.collider_type {
        //     ColliderType::Circle(radius) => { Vector2::new(b2_body.position().x, b2_body.position().y) },
        //     ColliderType::Box(scale) => { Vector2::new(b2_body.position().x - scale.x/2.0, b2_body.position().y - scale.y /2.0) }
        // };

        sprite_batch.add_to_batch(
            pos,
            scale,
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 1.0),
            self.color,
            self.texture_id,
            &angle,
            0.0
        );
    }
}
