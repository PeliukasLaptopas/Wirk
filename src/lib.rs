pub mod fps;
pub mod engine_error;
pub mod resources;
pub mod rendering;
pub mod input;
pub mod ecs;

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
use wrapped2d::dynamics::body::BodyType::Kinematic;

use rand::prelude::*;
use crate::rendering::sprite::Sprite;
use crate::rendering::sprite::rigid_body_2d::ColliderType;
use crate::rendering::vertex::vertex_data::u2_u10_u10_u10_rev_float;
use sdl2::video::{Window, GLContext};
use gl::Gl;
use sdl2::Sdl;

use legion::*;
use legion::systems::Builder;
use crate::ecs::Ecs;

pub struct Engine {
    pub opened: bool,
    pub input_manager: InputManager,
    pub window: Window,
    pub viewport: Viewport,
    pub color_buffer: ColorBuffer,
    pub sdl: Sdl,
    pub gl_context: GLContext, //if this isnt stored here it dies after create_window function and nothing will be drawn after the function is done
    pub gl: Rc<Gl>,
    pub ecs: Ecs
}

impl Engine {
    pub fn new(window_width: u32, window_height: u32, ecs: Ecs) -> Result<Engine, failure::Error> {
        let sdl = sdl2::init().map_err(err_msg)?;
        let video_subsystem = sdl.video().map_err(err_msg)?;
        let mut time_subsystem = sdl.timer().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_double_buffer(true); //i think its on by default
        gl_attr.set_context_version(4, 5);

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

        let mut viewport = Viewport::for_window(window_width as i32, window_height as i32);
        let color_buffer = ColorBuffer::from_color(Vector3::new(0.3, 0.3, 0.5));

        viewport.use_viewport(&gl);
        color_buffer.clear_color(&gl);

        let mut input_manager = InputManager::new(&sdl)?;

        Ok(Engine {
            opened: true,
            input_manager,
            window,
            viewport,
            color_buffer,
            sdl,
            gl_context,
            gl,
            ecs
        })
    }

    pub fn run(&mut self) {
        'main: loop {
            self.input_manager.update();
            if (!self.input_manager.window_opened) {
                break 'main;
            }

            self.ecs.run();

            self.color_buffer.clear(&self.gl);
            self.window.gl_swap_window();
        }
    }
}
