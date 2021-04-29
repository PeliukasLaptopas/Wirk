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
    Circle(f32),
    Box(Vec2)
}

impl RigidBody2D {
    /*fn create_physics_object(world: &mut World<NoUserData>, shape: ColliderType, body_type: &BodyType, position: Vec2) -> BodyHandle {
                let mut b_def = b2::BodyDef {
                    body_type: *body_type,
                    ..b2::BodyDef::new()
                };

                // let body = world.create_body(&b_def);

                let mut fixture = b2::FixtureDef {
                    density: 1.,
                    restitution: 0.6,
                    friction: 0.3,
                    ..b2::FixtureDef::new()
                };

                let body = match shape {
                    ColliderType::Circle(radius) => {
                        let body = world.create_body(&b_def);
                        b_def.position = Vec2 {
                            x: position.x,
                            y: position.y
                        };

                        let circle_shape = b2::CircleShape::new_with(Vec2{x:0.0, y:0.0}, radius / 2.0);
                        world.body_mut(body).create_fixture(&circle_shape, &mut fixture);
                        body
                    },
                    ColliderType::Box(scale) => {
                        let body = world.create_body(&b_def);
                        b_def.position = Vec2 {
                            x: position.x,
                            y: position.y
                        };

                        let polygon_shape = b2::PolygonShape::new_oriented_box(scale.x / 2.0, scale.y / 2.0, &Vec2 { x: 0.0 ,y: 0.0 }, 0.0);
                        world.body_mut(body).create_fixture(&polygon_shape, &mut fixture);
                        body
                    },
                };

                body
            }
     */

    fn create_physics_object(world: &mut World<NoUserData>, shape: ColliderType, body_type: &BodyType, position: Vec2/*, angle: f32*/) -> BodyHandle {

        let mut fixture = b2::FixtureDef {
            density: 0.5,
            restitution: 0.6,
            friction: 0.9,
            ..b2::FixtureDef::new()
        };

        let body = match shape {
            ColliderType::Circle(radius) => {
                let mut b_def = b2::BodyDef {
                    body_type: *body_type,
                    position: Vec2 {
                        x: position.x,
                        y: position.y
                    },
                    ..b2::BodyDef::new()
                };

                let body = world.create_body(&b_def);

                let circle_shape = b2::CircleShape::new_with(Vec2{x:0.0, y:0.0}, radius / 2.0);
                world.body_mut(body).create_fixture(&circle_shape, &mut fixture);

                body
            },
            ColliderType::Box(scale) => {
                let mut b_def = b2::BodyDef {
                    body_type: *body_type,
                    position: Vec2 {
                        x: position.x,
                        y: position.y
                    },
                    ..b2::BodyDef::new()
                };

                let body = world.create_body(&b_def);

                let polygon_shape = b2::PolygonShape::new_box(scale.x / 2.0, scale.y / 2.0/*, &Vec2 { x: 0.0 ,y: 0.0 }, 0.0*/);
                world.body_mut(body).create_fixture(&polygon_shape, &mut fixture);

                body
            },
        };

        body
    }

    pub fn new_circle_body(world: &mut World<NoUserData>, body_type: &BodyType, position: Vec2, radius: f32) -> RigidBody2D {
        let body = RigidBody2D::create_physics_object(world, Circle(radius), body_type, position);
        RigidBody2D {
            collider_type: Circle(radius),
            body,
        }
    }

    pub fn new_box_body(world: &mut World<NoUserData>, body_type: &BodyType, position: Vec2, scale: Vec2) -> RigidBody2D {
        let body = RigidBody2D::create_physics_object(world, Box(scale), body_type, position);

        // body.set_linear_velocity();

        RigidBody2D {
            collider_type: Box(scale),
            body,
        }
    }
}