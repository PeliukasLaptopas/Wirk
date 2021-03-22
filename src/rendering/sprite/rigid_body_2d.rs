use wrapped2d::b2::{BodyHandle, Shape, BodyType, CircleShape, World, PolygonShape};
use wrapped2d::b2;
use wrapped2d::common::math::Vec2;
use wrapped2d::user_data::NoUserData;
use core::fmt::Alignment::Right;
use crate::rendering::sprite::rigid_body_2d::ColliderType::Circle;
use crate::rendering::sprite::rigid_body_2d::ColliderType::Box;

pub struct RigidBody2D {
    pub collider_type: ColliderType,
    pub body: BodyHandle,
}

pub enum ColliderType {
    Circle,
    Box
}

impl RigidBody2D {
    fn create_physics_object(world: &mut World<NoUserData>, shape: ColliderType, body_type: &BodyType, position: Vec2, scale: f32) -> BodyHandle {
        let mut b_def = b2::BodyDef {
            body_type: *body_type,
            position,
            ..b2::BodyDef::new()
        };

        let body = world.create_body(&b_def);

        let mut fixture = b2::FixtureDef {
            density: 1.,
            restitution: 0.2,
            friction: 0.3,
            ..b2::FixtureDef::new()
        };

        match shape {
            ColliderType::Circle => {
                let circle_shape = b2::CircleShape::new_with(position, scale);
                world.body_mut(body).create_fixture(&circle_shape, &mut fixture);
            },
            ColliderType::Box => {
                let polygon_shape = b2::PolygonShape::new_box(scale, scale);
                world.body_mut(body).create_fixture(&polygon_shape, &mut fixture);
            },
        };

        body
    }

    pub fn new_circle_body(world: &mut World<NoUserData>, body_type: &BodyType, position: Vec2, scale: f32) -> RigidBody2D {
        let body = RigidBody2D::create_physics_object(world, Circle, body_type, position, scale);
        RigidBody2D {
            collider_type: Circle,
            body,
        }
    }
}