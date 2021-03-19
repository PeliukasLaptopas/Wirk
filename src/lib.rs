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
    // let sprite1 = sprite::Sprite::new(&Vector2::new(0.0, 0.0), &Vector2::new(200.0, 200.0), "water.png", &mut res, &gl)?;
    // let sprite2 = sprite::Sprite::new(&Vector2::new(200.0, 200.0), &Vector2::new(200.0, 200.0), "water.png", &mut res, &gl)?;

    let mut sprite_batch = SpriteBatch::new(&gl);

    viewport.use_viewport(&gl);
    color_buffer.clear_color(&gl);

    let mut time: f32 = 1.0;

    let mut fps_calculator = fps::FpsCalculator::new();
    let max_fps: u32 = 60;

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    let mut start_ticks: u32 = 0;

    let mut camera_pos = Vector2::new(50.0, 50.0);
    let mut camera = Camera2D::new(camera_pos, 1.0, window_width, window_height);

    let texture_id = res.get_texture("Character.png", &gl)?; //todo should get width and height from this function and store that here in sprite
    let mut program = Program::from_res(&gl, &mut res, "shaders/triangle")?;

    let mut input_manager = InputManager::new();

    'main: loop {
        start_ticks = time_subsystem.ticks();

        unsafe {
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        fps_calculator.start(&mut time_subsystem);
        time += 0.1;


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
                _ => {}
            }
        }

        let speed: f32 = 10.0;
        if (input_manager.is_key_pressed(&Keycode::A)) {
            camera_pos -= Vector2::new(speed, 0.0);
            camera.set_position(camera_pos);
        }

        if (input_manager.is_key_pressed(&Keycode::D)) {
            camera_pos += Vector2::new(speed, 0.0);
            camera.set_position(camera_pos);
        }

        if (input_manager.is_key_pressed(&Keycode::W)) {
            camera_pos += Vector2::new(0.0, speed);
            camera.set_position(camera_pos);
        }

        if (input_manager.is_key_pressed(&Keycode::S)) {
            camera_pos -= Vector2::new(0.0, speed);
            camera.set_position(camera_pos);
        }

        color_buffer.clear(&gl);

        camera.update();


        // sprite1.draw(&mut camera, &gl, &&time);
        // sprite2.draw(&mut camera, &gl, &&time);

        sprite_batch.begin();

        // for i in 0..10000 {
            sprite_batch.add_to_batch(
                Vector4::new(50.0, 50.0, 60.0, 100.0),
                Vector4::new(0.0, 0.0, 1.0, 1.0),
                (1.0, 1.0, 1.0, 1.0).into(),
                texture_id, 0.0
            );
        // }

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