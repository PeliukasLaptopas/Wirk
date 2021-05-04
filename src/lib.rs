pub mod fps;
pub mod engine_error;
pub mod resources;
pub mod rendering;
pub mod input;
pub mod ecs;
pub mod sound;
pub mod tests;

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
use crate::input::{SdlInputManager, Input};
use rendering::vertex::*;

use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;
use wrapped2d::b2::{World, Vec2, BodyType};
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
use sdl2::rect::Rect;

use legion::*;
use legion::systems::Builder;
use crate::ecs::Ecs;
use crate::rendering::texture::Texture;
use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use crate::rendering::ui::text::{Text};
use crate::rendering::ui::ui_batch::UIBatch;
// use crate::rendering::ui::text::get_centered_rect;
// use sdl2::ttf::Sdl2TtfContext;
// use sdl2::surface::SurfaceRef;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV, AudioDevice};
use std::borrow::Cow;
use std::path::{PathBuf};
use sdl2::{AudioSubsystem};

pub struct Engine<'a> {
    pub opened: bool,
    pub input_manager: SdlInputManager,
    pub window: Window,
    pub viewport: Viewport,
    pub color_buffer: ColorBuffer,
    pub sdl: Sdl,
    pub gl_context: GLContext, //if this isnt stored here it dies after create_window function and nothing will be drawn after the function is done
    pub gl: Rc<Gl>,
    pub ecs: Ecs,
    pub camera: Camera2D,
    pub sprite_batch: SpriteBatch,
    pub ui_batch: SpriteBatch,
    pub program: Program,
    pub resources: Resources<'a>,
    pub physics_world: World<NoUserData>,
}

impl<'a> Engine<'_> {
    pub fn new(window_width: u32, window_height: u32, gravity: b2::Vec2, ecs: Ecs) -> Result<Engine<'a>, failure::Error> {
        let sdl = sdl2::init().map_err(err_msg)?;
        let video_subsystem = sdl.video().map_err(err_msg)?;
        let mut time_subsystem = sdl.timer().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_double_buffer(true); //i think its on by default
        gl_attr.set_context_version(4, 5);

        let window = video_subsystem
            .window("Alesis Zaidimas", window_width, window_height)
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

        let mut input_manager = SdlInputManager::new(&sdl)?;

        let mut camera_pos = Vector2::new(0.0, 0.0);
        let camera_scale = 32.0; //32 pixels per meter
        let mut camera = Camera2D::new(camera_pos, camera_scale, window_width, window_height);

        let mut resources = Resources::from_relative_path(Path::new("assets")).unwrap();

        let mut program = Program::from_res(&gl, &mut resources, "shaders/triangle")?;

        let mut sprite_batch = SpriteBatch::new(&gl);
        let mut ui_batch = SpriteBatch::new(&gl);

        let mut world: World<NoUserData> = b2::World::<NoUserData>::new(&gravity);

        Ok(Engine {
            opened: true,
            input_manager,
            window: window,
            viewport,
            color_buffer,
            sdl,
            gl_context,
            gl,
            ecs,
            camera,
            sprite_batch,
            ui_batch,
            program,
            resources,
            physics_world: world
        })
    }

    pub fn new_sprite(&mut self,
                      pos: Vector2<f32>,
                      body_type: &BodyType,
                      collider_type: ColliderType,
                      color: u2_u10_u10_u10_rev_float,
                      texture: &Texture
    ) -> Result<Sprite, failure::Error> {
            Sprite::new(pos, body_type, collider_type, color, &mut self.physics_world, texture)
    }

    pub fn new_text(&mut self,
                    // text: String,
                    pos: Vector2<f32>,
                    scale: Vector2<f32>,
                    color: u2_u10_u10_u10_rev_float,
                    texture: &Texture
    ) -> Text {
        Text::new(pos, scale, color, texture)
    }

    pub fn update_camera(&mut self) {
        self.camera.update();
    }

    pub fn render_sprites(&mut self) {
        let mut query = <(&Sprite)>::query();

        self.sprite_batch.begin();
        for (sprite) in query.iter_mut(&mut self.ecs.world) {
            let current_angle = self.physics_world.body_mut(sprite.rigid_body_2d.body).angle();
            sprite.draw(&mut self.physics_world, &mut self.sprite_batch, current_angle);
        }

        self.sprite_batch.end();
        self.sprite_batch.render_batch(&mut self.camera, &mut self.program, &self.gl);
    }

    pub fn render_ui(&mut self) {
        let mut query = <(&Text)>::query();

        self.ui_batch.begin();
        for (text) in query.iter_mut(&mut self.ecs.world) {
            text.draw(&mut self.ui_batch, 0.1);
        }

        self.ui_batch.end();
        self.ui_batch.render_batch(&mut self.camera, &mut self.program, &self.gl);
    }

    pub fn update_input(&mut self) {
        let mut query = <(&mut Input)>::query();

        for (input) in query.iter_mut(&mut self.ecs.world) {
            self.input_manager.update(input, &self.camera);
        }
    }

    // pub fn update_manager_resources(&mut self) {
    //     let mut manager = self.ecs.resources.get_mut::<Manager>().unwrap();
    //     for i in 0..(manager.entities_to_remove.len()) {
    //         self.ecs.world.remove(manager.entities_to_remove[i]);
    //         manager.entities_to_remove.remove(i);
    //     }
    // }

    pub fn update_input_resources(&mut self) {
        let mut input = self.ecs.resources.get_mut::<Input>().unwrap();
        self.input_manager.update(&mut input, &self.camera)

            // .map(|mut input| self.input_manager.update(&mut input, &self.camera));
    }

    pub fn run(&mut self) {
        // self.ecs.world.push((, ));

        while (self.input_manager.window_opened) {
            self.physics_world.step(1.0 / 60.0, 6, 2);

            unsafe {
                self.gl.Enable(gl::BLEND);
                self.gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            }

            // self.input_manager.update();
            // self.update_input();
            self.update_input_resources();
            // self.update_manager_resources();

            self.ecs.run();

            self.color_buffer.clear(&self.gl);

            self.update_camera();

            self.render_sprites();
            self.render_ui();

            self.window.gl_swap_window();
        }
    }
}
