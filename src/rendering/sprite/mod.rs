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
use crate::rendering::sprite::sprite_batch::SpriteBatch;
use wrapped2d::b2::{BodyHandle, World, BodyType};
use wrapped2d::user_data::NoUserData;
use wrapped2d::b2;
use nalgebra::{Vector4, Vector2};
use wrapped2d::wrap::Wrapped;

pub struct Sprite {
    pub texture_id: GLuint,
    b2_body: BodyHandle,
    scale: Vector2<f32>,
}

impl Sprite {
    pub fn new(
        pos: Vector2<f32>,
        scale: Vector2<f32>,
        texture_name: &'static str,
        body_type: &BodyType,
        world: &mut World<NoUserData>,
        res: &mut Resources<'static>,
        gl: &gl::Gl,
    ) -> Result<Sprite, failure::Error> {
        let texture_id = res.get_texture(texture_name, gl)?; //todo should get width and height from this function and store that here in sprite

        let mut b_def = b2::BodyDef {
            body_type: *body_type,
            position: b2::Vec2 { x: pos.x, y: pos.y },
            ..b2::BodyDef::new()
        };

        let body = world.create_body(&b_def);
        let shape = b2::PolygonShape::new_box(scale.x, scale.y);

        let mut fixture = b2::FixtureDef {
            density: 1.,
            restitution: 0.2,
            friction: 0.3,
            ..b2::FixtureDef::new()
        };

        let f = world.body_mut(body).create_fixture(&shape, &mut fixture);

        // match world.body(body).fixture(f).shape().

        Ok(Sprite {
            texture_id,
            b2_body: body,
            scale
        })
    }

    pub fn get_pos(&self, world: &World<NoUserData>) -> Vector2<f32> {
        Vector2::new(world.body(self.b2_body).position().x, world.body(self.b2_body).position().y)
    }

    // pub fn update_pos(&mut self, new_position: Vector2<f32>) {
        // self.pos = new_position;
    // }

    pub fn draw(&self, world: &mut World<NoUserData>, camera: &mut Camera2D, gl: &gl::Gl, sprite_batch: &mut SpriteBatch) {
        let b2_body = world.body_mut(self.b2_body);
        let pos = Vector2::new(b2_body.position().x, b2_body.position().y);

        sprite_batch.add_to_batch(
            pos,
            self.scale.clone().into(),
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 1.0),
            (1.0, 1.0, 1.0, 1.0).into(),
            self.texture_id,
            &0.0,
            0.0
        );
    }
}