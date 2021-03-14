pub mod fps;
pub mod engine_error;
pub mod resources;
pub mod rendering;

#[macro_use] extern crate gl_derive;
#[macro_use] extern crate failure;

extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;
extern crate nalgebra;
extern crate image;
extern crate time;

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

pub fn open_window() -> Result<(), failure::Error> {
    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;
    let mut time_subsystem = sdl.timer().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_double_buffer(true); //i think its on by default
    gl_attr.set_context_version(4, 5);
    gl_attr.int

    let window = video_subsystem
        .window("Game", 900, 700)
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

    let mut viewport = Viewport::for_window(900, 700);

    let color_buffer = ColorBuffer::from_color(Vector3::new(0.3, 0.3, 0.5));

    let mut res = Resources::from_relative_path(Path::new("assets")).unwrap();
    let sprite1 = sprite::Sprite::new(&Vector2::new(-0.5, -0.5), &Vector2::new(0.5, 0.5), "water.png", &mut res, &gl)?;
    let sprite2 = sprite::Sprite::new(&Vector2::new(-0.2, -0.2), &Vector2::new(0.3, 0.3), "water.png", &mut res, &gl)?;

    viewport.use_viewport(&gl);
    color_buffer.clear_color(&gl);

    let mut time: f32 = 1.0;

    let mut fps_calculator = fps::FpsCalculator::new();
    let max_fps: u32 = 60;

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    let mut start_ticks: u32 = 0;

    'main: loop {
        start_ticks = time_subsystem.ticks();

        fps_calculator.start(&mut time_subsystem);
        time += 0.1;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.use_viewport(&gl);
                }
                _ => {}
            }
        }

        color_buffer.clear(&gl);
        sprite1.draw(&gl, &time);
        sprite2.draw(&gl, &time);

        window.gl_swap_window();

        let frame_ticks = time_subsystem.ticks() - start_ticks;
        if ((1000.0 / max_fps as f32) > frame_ticks as f32) {
            time_subsystem.delay(((1000.0 / max_fps as f32) - frame_ticks as f32) as u32);
        }

        println!("FPS: {}", fps_calculator.calculate_fps(&mut time_subsystem));
    }

    Ok(())
}