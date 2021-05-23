pub mod fps;
pub mod engine_error;
pub mod resources;
pub mod rendering;
pub mod input;
pub mod ecs;
pub mod sound;
pub mod tests;
pub mod collision;

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
use wrapped2d::user_data::{NoUserData, UserDataTypes};
use wrapped2d::b2::{World, Vec2, BodyType, MetaBody, Body, BodyHandle};
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
use crate::collision::PointCollider2D;
use std::cell::{RefMut, Ref};
use wrapped2d::handle::TypedHandle;

use uuid::Uuid;

pub struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            // With channel type u8 the "silence" value is 128 (middle of the 0-2^8 range) so we need
            // to both fill in the silence and scale the wav data accordingly. Filling the silence
            // once the wav is finished is trivial, applying the volume is more tricky. We need to:
            // * Change the range of the values from [0, 255] to [-128, 127] so we can multiply
            // * Apply the volume by multiplying, this gives us range [-128*volume, 127*volume]
            // * Move the resulting range to a range centered around the value 128, the final range
            //   is [128 - 128*volume, 128 + 127*volume] – scaled and correctly positioned
            //
            // Using value 0 instead of 128 would result in clicking. Scaling by simply multiplying
            // would not give correct results.
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
        }
    }
}

pub struct CustomUserData;
impl UserDataTypes for CustomUserData {
    type BodyData = Option<Uuid>;
    type JointData = ();
    type FixtureData = ();
}

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
    pub sprite_batch: SpriteBatch,
    pub ui_batch: SpriteBatch,
    pub program: Program,
    pub resources: Resources<'a>,
    // pub physics_world: World<NoUserData>,
}

impl<'a> Engine<'_> {
    pub fn new(window_width: u32, window_height: u32, camera_pos: Vector2<f32>, gravity: b2::Vec2, mut ecs: Ecs) -> Result<Engine<'a>, failure::Error> {
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

        let camera_scale = 32.0; //32 pixels per meter
        let mut camera = Camera2D::new(camera_pos, camera_scale, window_width, window_height);

        let mut resources = Resources::from_relative_path(Path::new("assets")).unwrap();

        let mut program = Program::from_res(&gl, &mut resources, "shaders/triangle")?;

        let mut sprite_batch = SpriteBatch::new(&gl);
        let mut ui_batch = SpriteBatch::new(&gl);

        let mut world: World<CustomUserData> = b2::World::<CustomUserData>::new(&gravity);

        ecs.resources.insert(world);
        ecs.resources.insert(PointCollider2D::new());
        ecs.resources.insert(camera);

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
            sprite_batch,
            ui_batch,
            program,
            resources,
            // physics_world: world
        })
    }

    pub fn new_sprite(&mut self,
                      pos: Vector2<f32>,
                      body_type: &BodyType,
                      collider_type: ColliderType,
                      color: u2_u10_u10_u10_rev_float,
                      texture: &Texture
    ) -> Sprite {
        let mut physics_world = self.ecs.resources.get_mut::<World<CustomUserData>>().unwrap(); //unsafe yes, but without this creating a new sprite would be a result - I don't want that. And also, currently physics is by default on
        Sprite::new(pos, body_type, collider_type, color, &mut physics_world, texture)
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
        for mut camera_2d in self.ecs.resources.get_mut::<Camera2D>() {
            camera_2d.update();
        }
    }

    pub fn render_sprites(&mut self) {
        let mut query = <(&Sprite)>::query();

        self.sprite_batch.begin();
        for (sprite) in query.iter_mut(&mut self.ecs.world) {

            for mut physics_world in self.ecs.resources.get_mut::<World<CustomUserData>>() {
                let current_angle = physics_world.body_mut(sprite.rigid_body_2d.body).angle();
                sprite.draw(&mut physics_world, &mut self.sprite_batch, current_angle);
            }
        }

        self.sprite_batch.end();

        for mut camera_2d in self.ecs.resources.get_mut::<Camera2D>() {
            self.sprite_batch.render_batch(&mut camera_2d, &mut self.program, &self.gl);
        }
    }

    pub fn render_ui(&mut self) {
        let mut query = <(&Text)>::query();

        self.ui_batch.begin();
        for (text) in query.iter_mut(&mut self.ecs.world) {
            text.draw(&mut self.ui_batch, 0.1);
        }

        self.ui_batch.end();

        for mut camera_2d in self.ecs.resources.get_mut::<Camera2D>() {
            self.ui_batch.render_batch(&mut camera_2d, &mut self.program, &self.gl);

        }
    }

    pub fn update_input(&mut self) {
        let mut query = <(&mut Input)>::query();

        for (input) in query.iter_mut(&mut self.ecs.world) {

            for mut camera_2d in self.ecs.resources.get_mut::<Camera2D>() {
                self.input_manager.update(input, &camera_2d);
            }
        }
    }

    pub fn play_sound(&mut self, wav_file: &Cow<'static, Path>) -> Result<AudioDevice<Sound>, String> {
        let audio_subsystem = self.sdl.audio()?;

        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1), // mono
            samples: None,     // default
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            let wav = AudioSpecWAV::load_wav(wav_file).expect("Could not load test WAV file");

            let cvt = AudioCVT::new(
                wav.format,
                wav.channels,
                wav.freq,
                spec.format,
                spec.channels,
                spec.freq,
            )
                .expect("Could not convert WAV file");

            let data = cvt.convert(wav.buffer().to_vec());

            // initialize the audio callback
            Sound {
                data: data,
                volume: 0.25,
                pos: 0,
            }
        });

        device
    }

    pub fn update_input_resources(&mut self) {
        let mut input = self.ecs.resources.get_mut::<Input>().unwrap();

        for mut camera_2d in self.ecs.resources.get_mut::<Camera2D>() {
            self.input_manager.update(&mut input, &camera_2d)

        }

        // .map(|mut input| self.input_manager.update(&mut input, &self.camera));
    }

    pub fn update_point_collider_2D_resources(&mut self) {
        let mut point_collider_2d = self.ecs.resources.get_mut::<PointCollider2D>().unwrap();
        let mut physics_world = self.ecs.resources.get_mut::<World<CustomUserData>>().unwrap();

        for input in self.ecs.resources.get_mut::<Input>() {
            let p = b2::Vec2 { x: point_collider_2d.point.x, y: point_collider_2d.point.y };
            let d = b2::Vec2 { x: 0.001, y: 0.001 };
            let aabb = b2::AABB {
                lower: p - d,
                upper: p + d,
            };

            let mut result = None;
            // let physics = self
            {
                let mut callback = |body_h: b2::BodyHandle, fixture_h: b2::FixtureHandle| {
                    let body = physics_world.body(body_h);
                    let fixture = body.fixture(fixture_h);

                    if body.body_type() != b2::BodyType::Static && fixture.test_point(&p) {
                        result = Some(body_h);
                        false
                    } else {
                        true
                    }
                };

                physics_world.query_aabb(&mut callback, &aabb);
            }

            point_collider_2d.body_handle = result;
        }
    }

    pub fn run(&mut self) {
        // self.ecs.world.push((, ));

        // self.physics_world.set_contact_filter(Box::none());
        // self.ecs.resources.insert(self.physics_world);
        // self.ecs.resources.get_mut()


        while (self.input_manager.window_opened) {



            for mut physics_world in self.ecs.resources.get_mut::<World<CustomUserData>>() {
                physics_world.step(1.0 / 60.0, 6, 2);
            }

            unsafe {
                self.gl.Enable(gl::BLEND);
                self.gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            }

            // self.input_manager.update();
            self.update_input();
            self.update_input_resources();
            // self.update_manager_resources();

            self.update_point_collider_2D_resources();

            self.ecs.run();

            self.color_buffer.clear(&self.gl);

            self.update_camera();

            self.render_sprites();
            self.render_ui();

            self.window.gl_swap_window();
        }
    }
}
