pub mod fps;
pub mod engine_error;
pub mod resources;
pub mod rendering;
pub mod input;

#[macro_use] extern crate gl_derive;
#[macro_use] extern crate failure;

extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;
extern crate nalgebra;
extern crate image;
extern crate time;
extern crate nalgebra_glm;
extern crate wrapped2d;

use nalgebra as na;
use std::rc::Rc;
use std::path::Path;
use crate::resources::Resources;
use failure::err_msg;
use crate::rendering::shader::program::Program;
use crate::rendering::{vertex, sprite};
use crate::rendering::vertex::Vertex;
use crate::rendering::shader::buffer;
use crate::rendering::shader::buffer::{Buffer, BufferTypeArray, VertexArray};
use crate::rendering::shader::viewport::Viewport;
use crate::rendering::shader::color_buffer::ColorBuffer;
use na::*;
use std::ffi::CString;
use crate::resources::texture_cache::TextureCache;
use crate::fps::*;
use std::time::{Duration, SystemTime};
use rendering::sprite::sprite_batch::*;
use sdl2::event::Event;
use crate::rendering::camera_2d::*;
use sdl2::keyboard::Keycode;
use crate::input::InputManager;
use rendering::vertex::*;

use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;
use wrapped2d::b2::{World, Vec2};
use wrapped2d::dynamics::body::BodyType::Static;
use wrapped2d::dynamics::body::BodyType::Dynamic;

use rand::prelude::*;
use crate::rendering::sprite::Sprite;

pub fn open_window() -> Result<(), failure::Error> {
    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;
    let mut time_subsystem = sdl.timer().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_double_buffer(true); //i think its on by default
    gl_attr.set_context_version(4, 5);

    let window_width: u32 = 1200;
    let window_height: u32 = 1200;

    let window = video_subsystem
        .window("Game", window_width, window_height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = Rc::new(
        gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void)
    );

    unsafe {
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut viewport = Viewport::for_window(window_width as i32, window_height as i32);

    let color_buffer = ColorBuffer::from_color(Vector3::new(0.3, 0.3, 0.5));

    let mut res = Resources::from_relative_path(Path::new("assets")).unwrap();

    let mut sprite_batch = SpriteBatch::new(&gl);

    viewport.use_viewport(&gl);
    color_buffer.clear_color(&gl);

    let mut time: f32 = 1.0;

    let mut fps_calculator = fps::FpsCalculator::new();
    let max_fps: u32 = 60;

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    let mut start_ticks: u32 = 0;

    let mut camera_pos = Vector2::new(0.0, 0.0);
    let camera_scale = 32.0; //32 pixels per meter
    let mut camera = Camera2D::new(camera_pos, camera_scale, window_width, window_height);

    let mut program = Program::from_res(&gl, &mut res, "shaders/triangle")?;

    let mut input_manager = InputManager::new();

    //--------------------------
    let gravity = b2::Vec2 { x: 0., y: -10. };

    let mut world: World<NoUserData> = b2::World::<NoUserData>::new(&gravity);
    //--------------------------

    // let box_rigid_body_2d_1 = BoxRigidBody2D::new(&mut world, &Dynamic, Vec2 { x: 0.0, y: 0.0 }, 0.0);
    // let box_rigid_body_2d_2 = BoxRigidBody2D::new(&mut world, &Dynamic, Vec2 { x: 0.0, y: 0.0 }, 0.0);
    // let box_rigid_body_2d_3 = BoxRigidBody2D::new(&mut world, &Dynamic, Vec2 { x: 0.0, y: 0.0 }, 0.0);
    // let box_rigid_body_2d_4 = BoxRigidBody2D::new(&mut world, &Dynamic, Vec2 { x: 0.0, y: 0.0 }, 0.0);

    let sprite1 = sprite::Sprite::new(Vector2::new(15.0, 25.0), Vector2::new(0.6, 1.0), "Character.png", &Dynamic, &mut world, &mut res, &gl)?;
    let sprite2 = sprite::Sprite::new(Vector2::new(25.6, 25.0), Vector2::new(0.6, 1.0), "Character.png", &Static, &mut world, &mut res, &gl)?;
    let ground = sprite::Sprite::new(Vector2::new(0.0, 10.0), Vector2::new(60.0, 1.0), "water.png", &Static, &mut world, &mut res, &gl)?;
    let center_sprite = sprite::Sprite::new(Vector2::new(15.0, 15.0), Vector2::new(1.0, 1.0), "circle.png", &Dynamic, &mut world, &mut res, &gl)?;

    let mut rng = thread_rng();
    let mut sprites: Vec<Sprite> = vec![];
    for i in 0..1000 {
        // sprites.push(
        //     sprite::Sprite::new(Vector2::new(0.0 + rng.gen_range(0..29) as f32, 10.0 + rng.gen_range(0..30) as f32), Vector2::new(0.6, 1.0), "Character.png", &Dynamic, &mut world, &mut res, &gl)?
        // );
    }


    let mut new_p = Vec2 {
        x: 0.0,
        y: 10.0
    };

    'main: loop {
        world.step(1.0 / 60.0, 6, 2);

        // println!("Pos: {}; {}",
        //          world.body(sprite.b2_body).position().x,
        //          world.body(sprite.b2_body).position().y);

        // new_p.y -= 0.01;

        start_ticks = time_subsystem.ticks();

        unsafe {
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        fps_calculator.start(&mut time_subsystem);
        time -= 0.25;


        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.use_viewport(&gl);
                }
                Event::KeyDown { keycode: Some(key_code), repeat: false, .. } => {
                    input_manager.press_key(&key_code);
                }
                Event::KeyUp { keycode: Some(key_code), repeat: false, .. } => {
                    input_manager.release_key(&key_code);
                }
                Event::MouseMotion { x, y, .. } => {
                    input_manager.set_mouse_coord(Vector2::new(x, y))
                }
                _ => {}
            }
        }


        if (input_manager.is_key_pressed(&Keycode::Space)) {
            camera.set_scale(camera.scale + 1.0);
        }
        if (input_manager.is_key_pressed(&Keycode::LShift)) {
            camera.set_scale(camera.scale - 1.0);
        }

        let mouse_pos = Vector2::new(input_manager.screen_mouse_position.x, input_manager.screen_mouse_position.y);
        let mouse_pos_world = camera.convert_screen_to_world(Vector2::new(mouse_pos.x, mouse_pos.y));
        world.body_mut(sprite2.rigid_body_2d.body).set_transform(&Vec2 {x: mouse_pos_world.x, y: mouse_pos_world.y}, 0.0);



        /*let speed: f32 = 10.0;
        if (input_manager.is_key_pressed(&Keycode::A)) {
            player_pos -= Vector2::new(speed, 0.0);
            sprite1.update_pos(player_pos);
        }

        if (input_manager.is_key_pressed(&Keycode::D)) {
            player_pos += Vector2::new(speed, 0.0);
            sprite1.update_pos(player_pos);
        }

        if (input_manager.is_key_pressed(&Keycode::W)) {
            player_pos += Vector2::new(0.0, speed);
            sprite1.update_pos(player_pos);
        }

        if (input_manager.is_key_pressed(&Keycode::S)) {
            player_pos -= Vector2::new(0.0, speed);
            sprite1.update_pos(player_pos);
        }*/

        color_buffer.clear(&gl);

        camera.update();

        sprite_batch.begin();

        let scale = Vector2::new(0.6, 1.0);
        // for i in 0..5 {
            /*sprite_batch.add_to_batch(
                sprite.get_pos(&world),
                scale,
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 1.0),
                (1.0, 1.0, 1.0, 1.0).into(),
                sprite.texture_id,
                &time,
                0.0
            );*/
        // }

        // for spr in sprites.iter_mut() {
        //     spr.draw(&mut world, &mut camera, &gl, &mut sprite_batch, (1.0, 1.0, 1.0, 1.0).into());
        // }

        sprite1.draw(&mut world, &mut camera, &gl, &mut sprite_batch, (1.0, 1.0, 1.0, 1.0).into());
        sprite2.draw(&mut world, &mut camera, &gl, &mut sprite_batch, (1.0, 1.0, 1.0, 1.0).into());
        ground.draw(&mut world, &mut camera, &gl, &mut sprite_batch, (1.0, 1.0, 1.0, 1.0).into());
        center_sprite.draw(&mut world, &mut camera, &gl, &mut sprite_batch, (1.0, 0.0, 0.0, 1.0).into());

        sprite_batch.end();
        sprite_batch.render_batch(&time, &mut camera, &mut program, &gl);

        window.gl_swap_window();

        let frame_ticks = time_subsystem.ticks() - start_ticks;
        if ((1000.0 / max_fps as f32) > frame_ticks as f32) {
            time_subsystem.delay(((1000.0 / max_fps as f32) - frame_ticks as f32) as u32);
        }

        // println!("FPS: {}", fps_calculator.calculate_fps(&mut time_subsystem));
    }

    Ok(())
}