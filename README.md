Video game engine written in Rust. Supports 2d physics (Box2D engine) and sprite batching.






































I have downloaded SDL2 (SDL2-devel-2.0.14-VC.zip) from https://www.libsdl.org/download-2.0.php and added /x64/* files to C:\Users\*\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\ in order for SDL to work

Machine needs visual c++ installed. Just simply download visual studio 2019 or you will get this error while compiling:
"please ensure that VS 2013, VS 2015, VS 2017 or VS 2019 was installed with the Visual C++ option"
Just download vs from here:
https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=Community&rel=16
and install c++ option

Box2D requiriments
if cmake is not installed you will get this error while compiling:
![image](https://user-images.githubusercontent.com/38985678/112358177-c328fd00-8cd8-11eb-9e83-629307c38f25.png)

download and install cmake-3.20.0-windows-x86_64.msi
![image](https://user-images.githubusercontent.com/38985678/112358303-e489e900-8cd8-11eb-9814-ec333b93d74c.png)
This is a project that uses my engine
![image](https://user-images.githubusercontent.com/38985678/111040399-7a04bd80-843b-11eb-9734-81dddbca835a.png)

Supports
Vsync
fps calculator
fps limiter

Ball game:
```
use RustEngineLibrary::*;
use RustEngineLibrary::engine_error::failure_to_string;
use wrapped2d::b2::{World, BodyHandle, JointHandle, MetaBody};
use wrapped2d::b2::MouseJointDef;
use wrapped2d::b2::MouseJoint;
use wrapped2d::b2::Vec2;
use wrapped2d::user_data::{NoUserData, UserData};
use wrapped2d::b2;
use wrapped2d::dynamics::body::BodyType::Kinematic;
use wrapped2d::dynamics::body::BodyType::Dynamic;
use std::path::Path;
use crate::rendering::sprite::rigid_body_2d::ColliderType;
use nalgebra::Vector2;
use rendering::sprite::sprite_batch::*;
use crate::rendering::sprite::Sprite;
use crate::rendering::{vertex, sprite};
use crate::resources::Resources;
use crate::rendering::camera_2d::*;
use crate::rendering::shader::program::Program;
use legion::*;
use crate::ecs::Ecs;
use legion::systems::SystemFn;
use rand::{thread_rng, Rng};
use crate::rendering::ui::text::{Text};
use sdl2::keyboard::Keycode;
use crate::input::{Input};
use crate::rendering::sprite::rigid_body_2d::{RigidBody2D};
use std::num::NonZeroU64;
use legion::systems::CommandBuffer;
use legion::query::{Passthrough, ComponentFilter, And, EntityFilterTuple};
use std::borrow::Cow;
use wrapped2d::dynamics::joints::JointDef;
use wrapped2d::wrap::FromFFI;
use std::sync::{Arc, Mutex};
use wrapped2d::dynamics::joints::mouse::ffi::{MouseJoint_as_joint, Joint_as_mouse_joint};
use crate::collision::PointCollider2D;
use std::cell::RefMut;
use uuid::Uuid;
use legion::world::SubWorld;

struct Time(i32);

struct Gravity(Vec2);
struct Sound;

struct Player {
}

struct Tits {
    i: i32
}

fn main() -> Result<(), failure::Error> {

    // #[system(simple)]
    // fn collision(#[resource] physics_world: &mut b2::World::<NoUserData>) {
        // for mut physics_world in self.ecs.resources.get_mut::<World<NoUserData>>() {
        //     for input in self.ecs.resources.get_mut::<Input>() {
        //         let p = b2::Vec2 { x: input.world_mouse_position.x, y: input.world_mouse_position.y };
        //         let d = b2::Vec2 { x: 0.001, y: 0.001 };
        //         let aabb = b2::AABB {
        //             lower: p - d,
        //             upper: p + d,
        //         };
        //
        //         let mut result = None;
        //         // let physics = self
        //         {
        //             let mut callback = |body_h: b2::BodyHandle, fixture_h: b2::FixtureHandle| {
        //                 let body = physics_world.body(body_h);
        //                 let fixture = body.fixture(fixture_h);
        //
        //                 if body.body_type() != b2::BodyType::Static && fixture.test_point(&p) {
        //                     result = Some(body_h);
        //                     false
        //                 } else {
        //                     true
        //                 }
        //             };
        //             physics_world.query_aabb(&mut callback, &aabb);
        //         }
        //
        //         if !result.is_none() {
        //             println!("SOME");
        //         } else {
        //             println!("NONE");
        //         }
        //     }
        // }
    // }

    // #[system(for_each)]
    // fn get_keys(sprite: &Sprite, player: &Player, mouse_joint: &MouseJoint, #[resource] input: &mut Input) {
    //     if (input.is_key_pressed(&Keycode::Space)) {
    //         println!("{}", sprite.texture_id);
    //     }
    //
    //     println!("{};{}", input.world_mouse_position.x, input.world_mouse_position.y);
    // }

    // a system fn which loops through Position and Velocity components, and reads the Time shared resource
    // the #[system] macro generates a fn called update_positions_system() which will construct our system
    #[system(for_each)]
    fn foo(commands: &mut CommandBuffer, sprite: &Sprite, entity: &Entity, #[resource] input: &mut Input) {
        // println!("{};{}", input.world_mouse_position.x, input.world_mouse_position.y);
        // manager.entities_to_remove.push(entity);

        if (input.is_key_pressed(&Keycode::Space)) {
            commands.remove(*entity);
        }


        if (input.is_key_pressed(&Keycode::Left)) {

        }
    }

    // let mut system_one = SystemBuilder::<()>::new("TestSystem")
    //     .read_resource::<TestResource>()
    //     .with_query(<(Sprite,)>::query()
    //     .build(move |commands, world, resource, queries| {
    //         let mut count = 0;
    //         {
    //             for (entity, pos) in queries.iter_entities(&mut *world) {
    //
    //             }
    //         }
    //     });

    #[system(for_each)]
    #[read_component(usize)]
    #[read_component(Sprite)]
    #[write_component(bool)]
    fn foo_deez(
        world: &mut SubWorld,
        #[resource] grabbed: &mut Option<JointHandle>,
        #[resource] input: &mut Input,
        #[resource] point_collider_2d: &mut PointCollider2D,
        #[resource] physics_world: &mut World<CustomUserData>,
        sprite: &mut Sprite,
        player: &Player
    ) {
        point_collider_2d.point = Vec2 {
            x: input.world_mouse_position.x,
            y: input.world_mouse_position.y
        };

        if !point_collider_2d.body_handle.is_none() {
            if (input.on_key_down(&Keycode::Space)) {
                let mass;
                let center;
                {
                    let mut body: RefMut<MetaBody<CustomUserData>> = physics_world.body_mut(point_collider_2d.body_handle.unwrap());
                    mass = body.mass();
                    center = *body.world_center();
                    body.set_awake(true);

                    // construct a query from a "view tuple"
                    let mut query = <(&usize, &mut bool)>::query();

                    // this time we have &Velocity and &mut Position
                    for (a, b) in query.iter_mut(world) {
                        // sprite.color = (1.0, 0.0, 0.0, 1.0).into();
                    }
                }

                let mut j_def = b2::MouseJointDef::new(sprite.rigid_body_2d.body, point_collider_2d.body_handle.unwrap());
                j_def.target = center;
                j_def.max_force = 1000. * mass;

                *grabbed = Some(physics_world.create_joint(&j_def));
            }
        } else {
            // println!("NONE");
        }

        if (input.on_key_up(&Keycode::Space)) {
            if let Some(j) = grabbed.take() {
                physics_world.destroy_joint(j)
            }
            *grabbed = None;
        }

        if grabbed.is_some() {
            let mut j = physics_world.joint_mut(grabbed.unwrap());

            match **j {
                b2::UnknownJoint::Mouse(ref mut j) => {
                    j.set_target(&Vec2{x: input.world_mouse_position.x, y:input.world_mouse_position.y});
                }
                _ => panic!("expected mouse joint"),
            }
        }
    }

    let mut tits = Tits {i:69};

    let body_handle: Option<JointHandle> = None;

    let mut ecs_resources = legion::Resources::default();
    ecs_resources.insert(tits);
    ecs_resources.insert(Input::new());
    ecs_resources.insert(body_handle);

    let ecs_world = legion::World::default();

    let mut schedule = Schedule::builder()
        .add_thread_local(foo_deez_system())
        // .add_system(get_keys_system())
        // .add_system(delete_half_sprites_logic())
        .build();

    let ecs = Ecs {
        schedule: schedule,
        resources: ecs_resources,
        world: ecs_world
    };

    let width  = 1200;
    let height = 1200;
    let gravity = Vec2 {x: 0.0, y: -9.9 };

    let mut maybe_engine: Result<Engine, failure::Error> =
        Engine::new(width,
                    height,
                    gravity,
                    ecs);

    match maybe_engine {
        Ok(mut engine) => unsafe {
            let wall_texture = engine.resources.get_texture("water.png", &engine.gl)?;
            let character_texture = engine.resources.get_texture("character.png", &engine.gl)?;

            // let wav_file: Cow<'static, Path> = Cow::from(Path::new("./assets/laser.wav"));
            //
            // engine.play_sound(&wav_file).unwrap().resume();
            //
            // engine.resources.load_cstring("knyga.txt");

            //
            // let sprite_texture =
            //     engine.resources.get_texture("character.png",
            //                                 &engine.gl)?;

            let input = engine.ecs.resources.get_mut::<Input>()
                .map(|input|
                         println!("x y {}", input.world_mouse_position.x)
                );

            // let p = b2::Vec2 { x: 15.0, y: 15.0 };
            // let d = b2::Vec2 { x: 0.001, y: 0.001 };
            // let aabb = b2::AABB {
            //     lower: p - d,
            //     upper: p + d,
            // };



            let circle_texture = engine.resources.get_texture("circle.png", &engine.gl)?;

            let floor: Sprite = engine.new_sprite(
                Vector2::new(20.0, -0.5),
                &Kinematic,
                ColliderType::Box(Vec2 { x: 50.0, y: 1.0 }),
                (1.0, 1.0, 1.0, 1.0).into(),
                &wall_texture
            );

            let right_wall: Sprite = engine.new_sprite(
                Vector2::new(38.0, 0.0),
                &Kinematic,
                ColliderType::Box(Vec2 { x: 1.0, y: 100.0 }),
                (1.0, 1.0, 1.0, 1.0).into(),
                &wall_texture
            );

            let left_wall: Sprite = engine.new_sprite(
                Vector2::new(-0.5, 0.0),
                &Kinematic,
                ColliderType::Box(Vec2 { x: 1.0, y: 100.0 }),
                (1.0, 1.0, 1.0, 1.0).into(),
                &wall_texture
            );

            let position = Vector2::new(15.5, 20.0);
            let physics_type = Kinematic;
            let color = (1.0, 1.0, 1.0, 1.0);
            let box_collider = ColliderType::Box(Vec2 { x: 3.0, y: 5.0 });

            let sprite: Sprite = engine.new_sprite(
                position,
                &physics_type,
                box_collider,
                color.into(),
                &character_texture
            );

            // engine.ecs.world.push((text, ));

            let character: Sprite = engine.new_sprite(
                Vector2::new(20.0, 20.0),
                &Dynamic,
                ColliderType::Box(Vec2 { x: 0.8, y: 1.0 }),
                (1.0, 1.0, 1.0, 1.0).into(),
                &character_texture
            );

            let character1: Sprite = engine.new_sprite(
                Vector2::new(20.0, 20.0),
                &Dynamic,
                ColliderType::Box(Vec2 { x: 1.6, y: 2.0 }),
                (1.0, 1.0, 1.0, 1.0).into(),
                &character_texture
            );

            let character2: Sprite = engine.new_sprite(
                Vector2::new(20.0, 20.0),
                &Dynamic,
                ColliderType::Box(Vec2 { x: 1.6, y: 2.0 }),
                (1.0, 1.0, 1.0, 1.0).into(),
                &character_texture
            );

            // let mouse_joint_definition = b2::MouseJointDef::new(floor.rigid_body_2d.body, character.rigid_body_2d.body);

            // let joint = mouse_joint_definition.create(&mut engine.physics_world);
            // let mouse_joint: MouseJoint = MouseJoint::from_ffi(joint);




            // let mouse_j: *mut MouseJoint = Joint_as_mouse_joint(mouse_joint);

            // mouse_joint.set_target(&Vec2{x:10.0, y:10.0});

            // engine.physics_world.create_joint(mouse_joint_definition);

            engine.ecs.world.push((floor, ));
            engine.ecs.world.push((left_wall, ));
            engine.ecs.world.push((right_wall, ));

            engine.ecs.world.push((sprite, ));

            engine.ecs.world.push((character, Player{}));
            engine.ecs.world.push((character1,));
            engine.ecs.world.push((character2,));

            let mut rng = thread_rng();

            for i in 0..600 {
                let x = 0.0 + rng.gen_range(10.0..25.0) as f32;
                let y = 10.0 + rng.gen_range(0.0..150.0) as f32;
                let size = 0.0 + rng.gen_range(0.5..1.5) as f32;

                let color_r = 0.0 + rng.gen_range(0.9..1.0) as f32;
                let color_g = 0.0 + rng.gen_range(0.5..1.0) as f32;
                let color_b = 0.0 + rng.gen_range(0.9..1.0) as f32;

                let sprite: Sprite = engine.new_sprite(
                    Vector2::new(x, y),
                    &Dynamic,
                    ColliderType::Circle(size),
                    (color_r, color_g, color_b, 1.0).into(),
                    &circle_texture
                );

                engine.ecs.world.push((sprite,));
            }

            engine.run()
        },
        Err(e) => println!("{}", failure_to_string(e))
    }

    Ok(())
}








use RustEngineLibrary::*;
use RustEngineLibrary::engine_error::failure_to_string;
use wrapped2d::b2::{World, BodyHandle, JointHandle, MetaBody, WorldManifold, MAX_MANIFOLD_POINTS};
use wrapped2d::b2::MouseJointDef;
use wrapped2d::b2::MouseJoint;
use wrapped2d::b2::Vec2;
use wrapped2d::user_data::{NoUserData, UserData};
use wrapped2d::b2;
use wrapped2d::dynamics::body::BodyType::{Kinematic, Static};
use wrapped2d::dynamics::body::BodyType::Dynamic;
use std::path::Path;
use crate::rendering::sprite::rigid_body_2d::ColliderType;
use nalgebra::Vector2;
use rendering::sprite::sprite_batch::*;
use crate::rendering::sprite::Sprite;
use crate::rendering::{vertex, sprite};
use crate::resources::Resources;
use crate::rendering::camera_2d::*;
use crate::rendering::shader::program::Program;
use legion::*;
use crate::ecs::Ecs;
use legion::systems::SystemFn;
use rand::{thread_rng, Rng};
use crate::rendering::ui::text::{Text};
use sdl2::keyboard::Keycode;
use crate::input::{Input};
use crate::rendering::sprite::rigid_body_2d::{RigidBody2D};
use std::num::NonZeroU64;
use legion::systems::CommandBuffer;
use legion::query::{Passthrough, ComponentFilter, And, EntityFilterTuple};
use std::borrow::Cow;
use wrapped2d::dynamics::joints::JointDef;
use wrapped2d::wrap::FromFFI;
use std::sync::{Arc, Mutex};
use wrapped2d::dynamics::joints::mouse::ffi::{MouseJoint_as_joint, Joint_as_mouse_joint};
use crate::collision::PointCollider2D;
use std::cell::RefMut;
use uuid::Uuid;
use legion::world::SubWorld;
use nalgebra_glm::sin;
use gl::Gl;
use std::rc::Rc;
use wrapped2d::dynamics::contacts::ContactEdge;

struct Time(i32);

struct Gravity(Vec2);
struct Sound;

struct Player {
}

fn main() -> Result<(), failure::Error> {
    #[system(simple)]
    fn setup_environment(
        commands: &mut CommandBuffer,
        #[resource] resources: &mut Resources,
        #[resource] physics_world: &mut World<CustomUserData>,
        #[resource] gl: &mut Rc<Gl>,
    ) {
        let grass_texture = resources.get_texture("kenny\\Tiles\\tile_0018.png", &gl).unwrap();
        let player_texture = resources.get_texture("kenny\\Characters\\character_0004.png", &gl).unwrap();
        let circle_texture = resources.get_texture("circle.png", &gl).unwrap();

        let mut player_sprite = Sprite::new(Vector2::new(15.0, 5.0), &Dynamic, ColliderType::Box(Vec2{x: 3.0, y: 3.0}), (1.0, 1.0, 1.0, 1.0).into(), physics_world, &player_texture, 0.0);
        physics_world.body_mut(player_sprite.rigid_body_2d.body).set_rotation_fixed(true);
        physics_world.body_mut(player_sprite.rigid_body_2d.body).fixture_mut(player_sprite.rigid_body_2d.fixture_handle).set_friction(0.2);
        physics_world.body_mut(player_sprite.rigid_body_2d.body).fixture_mut(player_sprite.rigid_body_2d.fixture_handle).set_restitution(0.0);

        commands.push((player_sprite, Player{}));

        for x in -20..30 {
            let pos = x * 2;

            let grass_sprite = Sprite::new(Vector2::new(pos as f32, 1.0), &Static, ColliderType::Box(Vec2{x: 2.0, y: 2.0}), (1.0, 1.0, 1.0, 1.0).into(), physics_world, &grass_texture, 0.0);
            commands.push((grass_sprite,));
        }
    }

    #[system(for_each)]
    fn player_movement(
        #[resource] input: &mut Input,
        #[resource] physics_world: &mut World<CustomUserData>,
        sprite: &mut Sprite,
        player: &Player
    ) {
        let move_speed = 50.0;
        let jump_speed = 1500.0;

        if (input.is_key_pressed(&Keycode::A)) {
            physics_world.body_mut(sprite.rigid_body_2d.body).apply_force_to_center(&Vec2{x:-move_speed, y:0.0}, true);
        }
        if (input.is_key_pressed(&Keycode::D)) {
            physics_world.body_mut(sprite.rigid_body_2d.body).apply_force_to_center(&Vec2{x: move_speed, y:0.0}, true);
        }

        let pos = Vector2::new(physics_world.body(sprite.rigid_body_2d.body).position().x, physics_world.body(sprite.rigid_body_2d.body).position().y);

        let mut body = physics_world.body_mut(sprite.rigid_body_2d.body);

        let mut bellow = false;
        for (_, contact) in body.contacts() {
            if (contact.is_touching()) {
                let world_manifold = contact.world_manifold();

                for i in 0..MAX_MANIFOLD_POINTS {
                    if (world_manifold.points[i].y < pos.y - 3.0 / 2.0 + 0.01) {
                        bellow = true;
                    }
                }
            }
        }

        if (bellow) {
            if (input.on_key_down(&Keycode::Space)) {
                body.apply_force_to_center(&Vec2{x:0.0, y:jump_speed}, true);
            }
        }
    }



    #[system(simple)]
    fn create_ball_logic(
        commands: &mut CommandBuffer,
        #[resource] input: &mut Input,
        #[resource] physics_world: &mut World<CustomUserData>,
        #[resource] gl: &mut Rc<Gl>,
        #[resource] resources: &mut Resources,
    ) {
        let circle_texture = resources.get_texture("circle.png", &gl).unwrap();

        if (input.is_key_pressed(&Keycode::G)) {
            let mut rng = thread_rng();

            let size = 0.0 + rng.gen_range(0.5..1.0) as f32;

            let color_r = 0.0 + rng.gen_range(0.1..1.0) as f32;
            let color_g = 0.0 + rng.gen_range(0.5..1.0) as f32;
            let color_b = 0.0 + rng.gen_range(0.9..1.0) as f32;

            let ball: Sprite = Sprite::new(
                Vector2::new(input.world_mouse_position.x, input.world_mouse_position.y),
                &Dynamic,
                ColliderType::Circle(size),
                (color_r, color_g, color_b, 1.0).into(),
                physics_world,
                &circle_texture,
                0.0
            );

            commands.push((ball, ));
        }
    }



    #[system(simple)]
    #[read_component(usize)]
    #[write_component(Sprite)]
    #[write_component(bool)]
    fn foo_deez(
        commands: &mut CommandBuffer,
        world: &mut SubWorld,
        #[resource] camera: &mut Camera2D,
        #[resource] grabbed: &mut Option<JointHandle>,
        #[resource] grabbed_uuid: &mut Option<Uuid>,
        #[resource] input: &mut Input,
        #[resource] point_collider_2d: &mut PointCollider2D,
        #[resource] physics_world: &mut World<CustomUserData>,
        #[resource] time: &mut f32,
    ) {
        let dummy = physics_world.create_body(&b2::BodyDef::new());


        point_collider_2d.point = Vec2 {
            x: input.world_mouse_position.x,
            y: input.world_mouse_position.y
        };

        let camera_speed = 2.0;
        if (input.is_key_pressed(&Keycode::W)) {
            camera.set_position(Vector2::new(camera.position.x, camera.position.y + camera_speed));
        }
        if (input.is_key_pressed(&Keycode::S)) {
            camera.set_position(Vector2::new(camera.position.x, camera.position.y - camera_speed));
        }
        if (input.is_key_pressed(&Keycode::A)) {
            camera.set_position(Vector2::new(camera.position.x - camera_speed, camera.position.y));
        }
        if (input.is_key_pressed(&Keycode::D)) {
            camera.set_position(Vector2::new(camera.position.x + camera_speed, camera.position.y));
        }

        if (input.is_key_pressed(&Keycode::Z)) {
            println!("SCALE {}", camera.scale);
            camera.set_scale(camera.scale + 0.1);
        }
        if (input.is_key_pressed(&Keycode::X)) {
            camera.set_scale(camera.scale - 0.1);
        }

        if !point_collider_2d.body_handle.is_none() {
            if (input.on_key_down(&Keycode::E)) {
                let mass;
                let center;
                {
                    let mut body: RefMut<MetaBody<CustomUserData>> = physics_world.body_mut(point_collider_2d.body_handle.unwrap());
                    mass = body.mass();
                    center = *body.world_center();
                    body.set_awake(true);

                    let mut query = <(&mut Sprite)>::query();

                    let uuid = body.user_data_mut();

                    // this time we have &Velocity and &mut Position
                    for (mut sprite) in query.iter_mut(world) {
                        if sprite.uuid == uuid.unwrap() {
                            *grabbed_uuid = Some(sprite.uuid);
                        }
                    }
                }

                let mut j_def = b2::MouseJointDef::new(dummy, point_collider_2d.body_handle.unwrap());
                j_def.target = center;
                j_def.max_force = 1000. * mass;

                *grabbed = Some(physics_world.create_joint(&j_def));
            }
        } else {
            // println!("NONE");
        }

        if (input.on_key_up(&Keycode::Space)) {
            if let Some(j) = grabbed.take() {
                physics_world.destroy_joint(j)
            }
            *grabbed = None;
        }

        if grabbed.is_some() {
            let mut j = physics_world.joint_mut(grabbed.unwrap());

            match **j {
                b2::UnknownJoint::Mouse(ref mut j) => {
                    *time += 0.05;

                    let mut query = <(&mut Sprite)>::query();

                    // this time we have &Velocity and &mut Position
                    for (mut sprite) in query.iter_mut(world) {
                        if sprite.uuid == grabbed_uuid.unwrap() {
                            sprite.color = (1.0 * (time.cos() + 1.0) * 0.5, 1.0 * (time.sin() + 1.0) * 0.5, 0.07, 1.0).into();
                        }
                    }

                    j.set_target(&Vec2{x: input.world_mouse_position.x, y:input.world_mouse_position.y});
                }
                _ => panic!("expected mouse joint"),
            }
        }
    }

    let body_handle: Option<JointHandle> = None;
    let body_uuid: Option<Uuid> = None;

    let mut ecs_resources = legion::Resources::default();
    ecs_resources.insert(Input::new());
    ecs_resources.insert(body_handle);
    ecs_resources.insert(body_uuid);

    let ecs_world = legion::World::default();


    let mut startup_schedule = Schedule::builder()
        .add_thread_local(setup_environment_system())
        // .add_system(get_keys_system())
        // .add_system(delete_half_sprites_logic())
        .build();

    let mut logic_schedule = Schedule::builder()
        .add_thread_local(player_movement_system())
        .add_thread_local(create_ball_logic_system())
        // .add_system(get_keys_system())
        // .add_system(delete_half_sprites_logic())
        .build();


    let ecs = Ecs {
        schedule: logic_schedule,
        startup_schedule: startup_schedule,
        resources: ecs_resources,
        world: ecs_world
    };

    let width  = 1600;
    let height = 1200;
    let gravity = Vec2 {x: 0.0, y: -9.9 };

    let mut maybe_engine: Result<Engine, failure::Error> =
        Engine::new(width,
                    height,
                    Vector2::new(0.0, 0.0),
                    gravity,
                    ecs);

    match maybe_engine {
        Ok(mut engine) => unsafe {
            // let wall_texture = engine.resources.get_texture("water.png", &engine.gl)?;
            // let character_texture = engine.resources.get_texture("character.png", &engine.gl)?;
            // let water_texture = engine.resources.get_texture("water.png", &engine.gl)?;
            //
            // // let wav_file: Cow<'static, Path> = Cow::from(Path::new("./assets/laser.wav"));
            // //
            // // engine.play_sound(&wav_file).unwrap().resume();
            // //
            // // engine.resources.load_cstring("knyga.txt");
            //
            // //
            // // let sprite_texture =
            // //     engine.resources.get_texture("character.png",
            // //                                 &engine.gl)?;
            //
            // let input = engine.ecs.resources.get_mut::<Input>()
            //     .map(|input|
            //              println!("x y {}", input.world_mouse_position.x)
            //     );
            //
            // // let p = b2::Vec2 { x: 15.0, y: 15.0 };
            // // let d = b2::Vec2 { x: 0.001, y: 0.001 };
            // // let aabb = b2::AABB {
            // //     lower: p - d,
            // //     upper: p + d,
            // // };
            //
            // let circle_texture = engine.resources.get_texture("circle.png", &engine.gl)?;
            //
            // let floor: Sprite = engine.new_sprite(
            //     Vector2::new(15.0, -0.5),
            //     &Kinematic,
            //     ColliderType::Box(Vec2 { x: 50.0, y: 1.0 }),
            //     (1.0, 1.0, 1.0, 1.0).into(),
            //     &wall_texture
            // );
            //
            // let right_wall: Sprite = engine.new_sprite(
            //     Vector2::new(38.0, 0.0),
            //     &Kinematic,
            //     ColliderType::Box(Vec2 { x: 1.0, y: 100.0 }),
            //     (1.0, 1.0, 1.0, 1.0).into(),
            //     &wall_texture
            // );
            //
            // let left_wall: Sprite = engine.new_sprite(
            //     Vector2::new(-0.5, 0.0),
            //     &Kinematic,
            //     ColliderType::Box(Vec2 { x: 1.0, y: 100.0 }),
            //     (1.0, 1.0, 1.0, 1.0).into(),
            //     &wall_texture
            // );
            //
            // let sprite: Sprite = engine.new_sprite(
            //     Vector2::new(0.0, 0.0),
            //     &Static,
            //     ColliderType::Box(Vec2 { x: 1.0, y: 1.0 }),
            //     (1.0, 0.0, 0.0, 1.0).into(),
            //     &water_texture
            // );
            //
            // let sprite69: Sprite = engine.new_sprite(
            //     Vector2::new(0.0, 5.0),
            //     &Static,
            //     ColliderType::Box(Vec2 { x: 1.0, y: 1.0 }),
            //     (1.0, 0.0, 0.0, 1.0).into(),
            //     &water_texture
            // );
            //
            // let sprite2: Sprite = engine.new_sprite(
            //     Vector2::new(13.5, 21.0),
            //     &Static,
            //     ColliderType::Box(Vec2 { x: 3.0, y: 3.0 }),
            //     (1.0, 1.0, 1.0, 1.0).into(),
            //     &water_texture
            // );
            //
            // let sprite3: Sprite = engine.new_sprite(
            //     Vector2::new(10.5, 20.0),
            //     &Static,
            //     ColliderType::Box(Vec2 { x: 3.0, y: 3.0 }),
            //     (1.0, 1.0, 1.0, 1.0).into(),
            //     &water_texture
            // );
            //
            // // engine.ecs.world.push((text, ));
            //
            // let character: Sprite = engine.new_sprite(
            //     Vector2::new(20.0, 20.0),
            //     &Dynamic,
            //     ColliderType::Box(Vec2 { x: 0.8, y: 1.0 }),
            //     (0.0, 1.0, 0.0, 1.0).into(),
            //     &character_texture
            // );
            //
            // let character1: Sprite = engine.new_sprite(
            //     Vector2::new(20.0, 20.0),
            //     &Dynamic,
            //     ColliderType::Box(Vec2 { x: 1.6, y: 2.0 }),
            //     (1.0, 1.0, 1.0, 1.0).into(),
            //     &character_texture
            // );
            //
            // let character2: Sprite = engine.new_sprite(
            //     Vector2::new(20.0, 20.0),
            //     &Dynamic,
            //     ColliderType::Box(Vec2 { x: 1.6, y: 2.0 }),
            //     (1.0, 1.0, 1.0, 1.0).into(),
            //     &character_texture
            // );
            //
            // // let mouse_joint_definition = b2::MouseJointDef::new(floor.rigid_body_2d.body, character.rigid_body_2d.body);
            //
            // // let joint = mouse_joint_definition.create(&mut engine.physics_world);
            // // let mouse_joint: MouseJoint = MouseJoint::from_ffi(joint);
            //
            //
            //
            //
            // // let mouse_j: *mut MouseJoint = Joint_as_mouse_joint(mouse_joint);
            //
            // // mouse_joint.set_target(&Vec2{x:10.0, y:10.0});
            //
            // // engine.physics_world.create_joint(mouse_joint_definition);
            //
            //
            // engine.ecs.world.push((tits, ));
            //
            // engine.ecs.world.push((floor, ));
            // engine.ecs.world.push((left_wall, ));
            // engine.ecs.world.push((right_wall, ));
            //
            // engine.ecs.world.push((sprite69, ));
            // engine.ecs.world.push((sprite, ));
            // engine.ecs.world.push((sprite2, ));
            // engine.ecs.world.push((sprite3, ));
            //
            // engine.ecs.world.push((character, Player{}));
            // engine.ecs.world.push((character1,));
            // engine.ecs.world.push((character2,));
            //
            // let mut rng = thread_rng();
            //
            // for i in 0..500 {
            //     let x = 0.0 + rng.gen_range(10.0..25.0) as f32;
            //     let y = 10.0 + rng.gen_range(0.0..250.0) as f32;
            //     let size = 0.0 + rng.gen_range(0.5..1.0) as f32;
            //
            //     let color_r = 0.0 + rng.gen_range(0.9..1.0) as f32;
            //     let color_g = 0.0 + rng.gen_range(0.5..1.0) as f32;
            //     let color_b = 0.0 + rng.gen_range(0.9..1.0) as f32;
            //
            //     let sprite: Sprite = engine.new_sprite(
            //         Vector2::new(x, y),
            //         &Dynamic,
            //         ColliderType::Circle(size),
            //         (color_r, color_g, color_b, 1.0).into(),
            //         &circle_texture
            //     );
            //
            //     engine.ecs.world.push((sprite,));
            // }

            engine.run()
        },
        Err(e) => println!("{}", failure_to_string(e))
    }

    Ok(())
}


```



```use RustEngineLibrary::*;
use RustEngineLibrary::engine_error::failure_to_string;
use wrapped2d::b2::{World, BodyHandle, JointHandle, MetaBody, WorldManifold, MAX_MANIFOLD_POINTS};
use wrapped2d::b2::MouseJointDef;
use wrapped2d::b2::MouseJoint;
use wrapped2d::b2::Vec2;
use wrapped2d::user_data::{NoUserData, UserData};
use wrapped2d::b2;
use wrapped2d::dynamics::body::BodyType::{Kinematic, Static};
use wrapped2d::dynamics::body::BodyType::Dynamic;
use std::path::Path;
use crate::rendering::sprite::rigid_body_2d::ColliderType;
use nalgebra::Vector2;
use nalgebra::Vector3;
use rendering::sprite::sprite_batch::*;
use crate::rendering::sprite::Sprite;
use crate::rendering::sprite::PassiveSprite;
use crate::rendering::{vertex, sprite};
use crate::resources::Resources;
use crate::rendering::camera_2d::*;
use crate::rendering::shader::program::Program;
use legion::*;
use crate::ecs::Ecs;
use legion::systems::SystemFn;
use rand::{thread_rng, Rng};
use crate::rendering::ui::text::{Text};
use sdl2::keyboard::Keycode;
use crate::input::{Input};
use crate::rendering::sprite::rigid_body_2d::{RigidBody2D};
use std::num::NonZeroU64;
use legion::systems::CommandBuffer;
use legion::query::{Passthrough, ComponentFilter, And, EntityFilterTuple};
use std::borrow::Cow;
use wrapped2d::dynamics::joints::JointDef;
use wrapped2d::wrap::FromFFI;
use std::sync::{Arc, Mutex};
use wrapped2d::dynamics::joints::mouse::ffi::{MouseJoint_as_joint, Joint_as_mouse_joint};
use crate::collision::PointCollider2D;
use std::cell::RefMut;
use uuid::Uuid;
use legion::world::SubWorld;
use nalgebra_glm::sin;
use gl::Gl;
use std::rc::Rc;
use wrapped2d::dynamics::contacts::ContactEdge;

struct Player {
}

struct PlayGroundObject {
}

fn main() -> Result<(), failure::Error> {
    #[system(simple)]
    fn setup_environment(
        commands: &mut CommandBuffer,
        #[resource] resources: &mut Resources,
        #[resource] physics_world: &mut World<CustomUserData>,
        #[resource] gl: &mut Rc<Gl>,
    ) {
        let grass_texture = resources.get_texture("kenny2\\Tiles\\grassMid.png", &gl).unwrap();
        let player_texture = resources.get_texture("kenny2\\Enemies\\blockerSad.png", &gl).unwrap();
        let ground_texture = resources.get_texture("kenny\\Tiles\\ground.png", &gl).unwrap();
        let box_texture = resources.get_texture("kenny2\\Tiles\\boxAlt.png", &gl).unwrap();

        let mut player_sprite = Sprite::new(Vector2::new(15.0, 10.0), &Dynamic, ColliderType::Box(Vec2{x: 2.0, y: 2.0}), (1.0, 1.0, 1.0, 1.0).into(), physics_world, &player_texture, 0.0);
        physics_world.body_mut(player_sprite.rigid_body_2d.body).set_rotation_fixed(false);
        physics_world.body_mut(player_sprite.rigid_body_2d.body).fixture_mut(player_sprite.rigid_body_2d.fixture_handle).set_friction(0.0);
        physics_world.body_mut(player_sprite.rigid_body_2d.body).fixture_mut(player_sprite.rigid_body_2d.fixture_handle).set_restitution(0.0);

        //              0             1     2
        // sprites[] { player_sprite,    ,     }
        // players[] { Player{}     ,    ,     }

        commands.push((player_sprite, Player{}));

        for x in -20..30 {
            let pos = x * 2;

            let grass_sprite  = Sprite::new(Vector2::new(pos as f32, 2.0), &Static, ColliderType::Box(Vec2{x: 2.0, y: 2.0}), (1.0, 1.0, 1.0, 1.0).into(), physics_world, &grass_texture, 0.0);
            commands.push((grass_sprite,));
        }

        for x in -20..30 {
            for y in -0..30 {
                let pos_x = x * 2;
                let pos_y = -(y * 2);

                let ground_sprite  = Sprite::new(Vector2::new(pos_x as f32, pos_y as f32), &Static, ColliderType::Box(Vec2{x: 2.0, y: 2.0}), (1.0, 1.0, 1.0, 1.0).into(), physics_world, &ground_texture, 0.0);
                commands.push((ground_sprite,));
            }
        }

        for y in 2..15 {
            let pos_y = (y * 2);

            let ground_sprite_left = Sprite::new(Vector2::new(1.0, pos_y as f32), &Static, ColliderType::Box(Vec2{x: 2.0, y: 2.0}), (1.0, 1.0, 1.0, 1.0).into(), physics_world, &box_texture, 0.0);
            let ground_sprite_right = Sprite::new(Vector2::new(49.0, pos_y as f32), &Static, ColliderType::Box(Vec2{x: 2.0, y: 2.0}), (1.0, 1.0, 1.0, 1.0).into(), physics_world, &box_texture, 0.0);
            commands.push((ground_sprite_left,));
            commands.push((ground_sprite_right,));
        }
    }

    #[system(for_each)]
    fn player_movement(
        #[resource] input: &mut Input,
        #[resource] physics_world: &mut World<CustomUserData>,
        sprite: &mut Sprite,
        player: &Player
    ) {
        let move_speed = 100.0;
        let jump_speed = 1500.0;

        if (input.is_key_pressed(&Keycode::A)) {
            physics_world.body_mut(sprite.rigid_body_2d.body).apply_force_to_center(&Vec2{x:-move_speed, y:0.0}, true);
        }
        if (input.is_key_pressed(&Keycode::D)) {
            physics_world.body_mut(sprite.rigid_body_2d.body).apply_force_to_center(&Vec2{x: move_speed, y:0.0}, true);
        }
        if (input.on_key_down(&Keycode::Space)) {
            physics_world.body_mut(sprite.rigid_body_2d.body).apply_force_to_center(&Vec2{x:0.0, y:jump_speed}, true);
        }
    }

    #[system(simple)]
    fn camera(
        #[resource] input: &mut Input,
        #[resource] camera: &mut Camera2D,
    ) {
        let move_speed = 1.0;
        let zoom_speed = 0.3;

        if (input.is_key_pressed(&Keycode::Left)) {
            camera.set_position(Vector2::new(camera.position.x - move_speed, camera.position.y));
        }
        if (input.is_key_pressed(&Keycode::Right)) {
            camera.set_position(Vector2::new(camera.position.x + move_speed, camera.position.y));
        }
        if (input.is_key_pressed(&Keycode::Up)) {
            camera.set_position(Vector2::new(camera.position.x, camera.position.y + move_speed));
        }
        if (input.is_key_pressed(&Keycode::Down)) {
            camera.set_position(Vector2::new(camera.position.x, camera.position.y - move_speed));
        }

        if (input.is_key_pressed(&Keycode::X)) {
            camera.set_scale(camera.scale - zoom_speed);
        }
        if (input.is_key_pressed(&Keycode::Z)) {
            camera.set_scale(camera.scale + zoom_speed);
        }
    }

    #[system(simple)]
    fn create_squares_balls_logic(
        commands: &mut CommandBuffer,
        #[resource] input: &mut Input,
        #[resource] physics_world: &mut World<CustomUserData>,
        #[resource] gl: &mut Rc<Gl>,
        #[resource] resources: &mut Resources,
    ) {
        let circle_texture = resources.get_texture("kenny2\\HUD\\hud_p1.png", &gl).unwrap();
        let square_texture = resources.get_texture("kenny\\Tiles\\tile_0104.png", &gl).unwrap();

        if (input.is_key_pressed(&Keycode::G)) {
            let mut rng = thread_rng();

            let size = 0.0 + rng.gen_range(0.5..2.0) as f32;

            let color_r = 0.0 + rng.gen_range(0.1..1.0) as f32;
            let color_g = 0.0 + rng.gen_range(0.5..1.0) as f32;
            let color_b = 0.0 + rng.gen_range(0.9..1.0) as f32;

            let ball: Sprite = Sprite::new(
                Vector2::new(input.world_mouse_position.x, input.world_mouse_position.y),
                &Dynamic,
                ColliderType::Circle(size),
                (color_r, color_g, color_b, 1.0).into(),
                physics_world,
                &circle_texture,
                0.0
            );

            commands.push((ball, PlayGroundObject {}));
        }

        if (input.is_key_pressed(&Keycode::H)) {
            let mut rng = thread_rng();

            let size = 0.0 + rng.gen_range(0.5..2.0) as f32;

            let color_r = 0.0 + rng.gen_range(0.1..1.0) as f32;
            let color_g = 0.0 + rng.gen_range(0.5..1.0) as f32;
            let color_b = 0.0 + rng.gen_range(0.9..1.0) as f32;

            let ball: Sprite = Sprite::new(
                Vector2::new(input.world_mouse_position.x, input.world_mouse_position.y),
                &Dynamic,
                ColliderType::Box(Vec2{x:size, y:size}),
                (color_r, color_g, color_b, 1.0).into(),
                physics_world,
                &square_texture,
                0.0
            );

            commands.push((ball, PlayGroundObject {}));
        }
    }

    #[system(simple)]
    #[read_component(usize)]
    #[write_component(Sprite)]
    #[write_component(bool)]
    fn grab_logic(
        world: &mut SubWorld,
        #[resource] grabbed: &mut Option<JointHandle>,
        #[resource] grabbed_uuid: &mut Option<Uuid>,
        #[resource] input: &mut Input,
        #[resource] point_collider_2d: &mut PointCollider2D,
        #[resource] physics_world: &mut World<CustomUserData>,
        #[resource] time: &mut f32,
    ) {
        let dummy = physics_world.create_body(&b2::BodyDef::new());

        point_collider_2d.point = Vec2 {
            x: input.world_mouse_position.x,
            y: input.world_mouse_position.y
        };

        if !point_collider_2d.body_handle.is_none() {
            if (input.on_key_down(&Keycode::E)) {
                let mass;
                let center;
                {
                    let mut body: RefMut<MetaBody<CustomUserData>> = physics_world.body_mut(point_collider_2d.body_handle.unwrap());
                    mass = body.mass();
                    center = *body.world_center();
                    body.set_awake(true);

                    let mut query = <(&mut Sprite)>::query();

                    let uuid = body.user_data_mut();

                    // this time we have &Velocity and &mut Position
                    for (mut sprite) in query.iter_mut(world) {
                        if sprite.uuid == uuid.unwrap() {
                            *grabbed_uuid = Some(sprite.uuid);
                        }
                    }
                }

                let mut j_def = b2::MouseJointDef::new(dummy, point_collider_2d.body_handle.unwrap());
                j_def.target = center;
                j_def.max_force = 1000. * mass;

                *grabbed = Some(physics_world.create_joint(&j_def));
            }
        }

        if (input.on_key_up(&Keycode::E)) {
            if let Some(j) = grabbed.take() {
                physics_world.destroy_joint(j)
            }
            *grabbed = None;
        }

        if grabbed.is_some() {
            let mut j = physics_world.joint_mut(grabbed.unwrap());

            match **j {
                b2::UnknownJoint::Mouse(ref mut j) => {
                    *time += 0.05;

                    let mut query = <(&mut Sprite)>::query();

                    // this time we have &Velocity and &mut Position
                    for (mut sprite) in query.iter_mut(world) {
                        if sprite.uuid == grabbed_uuid.unwrap() {
                            sprite.color = (1.0 * (time.cos() + 1.0) * 0.5, 1.0 * (time.sin() + 1.0) * 0.5, 0.07, 1.0).into();
                        }
                    }

                    j.set_target(&Vec2{x: input.world_mouse_position.x, y:input.world_mouse_position.y});
                }
                _ => panic!("expected mouse joint"),
            }
        }
    }

    let body_handle: Option<JointHandle> = None;
    let body_uuid: Option<Uuid> = None;
    let time: f32 = 0.0;

    let mut ecs_resources = legion::Resources::default();
    ecs_resources.insert(Input::new());
    ecs_resources.insert(body_handle);
    ecs_resources.insert(body_uuid);
    ecs_resources.insert(time);

    let ecs_world = legion::World::default();

    let mut startup_schedule = Schedule::builder()
        .add_thread_local(setup_environment_system())
        .build();

    let mut logic_schedule = Schedule::builder()
        .add_thread_local(player_movement_system())
        .add_thread_local(create_squares_balls_logic_system())
        .add_thread_local(camera_system())
        .add_thread_local(grab_logic_system())
        .build();


    let ecs = Ecs {
        schedule: logic_schedule,
        startup_schedule: startup_schedule,
        resources: ecs_resources,
        world: ecs_world
    };

    let width  = 1600;
    let height = 1200;
    let gravity = Vec2 {x: 0.0, y: -9.9 };

    let mut maybe_engine: Result<Engine, failure::Error> =
        Engine::new(width,
                    height,
                    Vector2::new(0.0, 0.0),
                    gravity,
                    ecs);

    match maybe_engine {
        Ok(mut engine) => {
            engine.run()
        },
        Err(e) => println!("{}", failure_to_string(e))
    }

    Ok(())
}
```
