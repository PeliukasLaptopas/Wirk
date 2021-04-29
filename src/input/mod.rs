use std::collections::HashMap;
use sdl2::event::{EventPollIterator, Event};
use sdl2::{EventPump, Sdl};
use sdl2::keyboard::Keycode;
use nalgebra::Vector2;
use failure::err_msg;
use crate::rendering::camera_2d::Camera2D;

pub struct Input {
    key_map: HashMap<Keycode, bool>,
    pub screen_mouse_position: Vector2<i32>,
    pub world_mouse_position: Vector2<f32>
}

impl Input {
    pub fn new() -> Input {
        Input {
            key_map: HashMap::new(),
            screen_mouse_position: Vector2::new(0, 0),
            world_mouse_position: Vector2::new(0.0, 0.0)
        }
    }

    pub fn update_keyboard(&mut self, key_code: Keycode, val: bool) {
        self.key_map.insert(key_code, val);
    }

    pub fn update_screen_mouse(&mut self, mouse_position: Vector2<i32>) {
        self.screen_mouse_position = mouse_position;
    }

    pub fn update_world_mouse(&mut self, mouse_position: Vector2<f32>) {
        self.world_mouse_position = mouse_position;
    }

    pub fn is_key_pressed(&self, key_id: &Keycode) -> bool {
        if self.key_map.contains_key(key_id) {
            self.key_map[key_id]
        } else {
            false
        }
    }
}

pub struct SdlInputManager {
    key_map: HashMap<Keycode, bool>,
    pub event_pump: EventPump,
    pub screen_mouse_position: Vector2<i32>,
    pub window_opened: bool,
}

impl SdlInputManager {
    pub fn new(sdl: &Sdl) -> Result<SdlInputManager, failure::Error> {
        let mut event_pump = sdl.event_pump().map_err(err_msg)?;

        Ok(SdlInputManager {
            key_map: HashMap::new(),
            event_pump,
            screen_mouse_position: Vector2::new(0, 0),
            window_opened: true
        })
    }

    pub fn update(&mut self, input: &mut Input, camera: &Camera2D) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.window_opened = false,
                // Event::Window { win_event: sdl2::event::WindowEvent::Resized(w, h), .. } => { //todo
                //     self.viewport.update_size(w, h);
                //     self.viewport.use_viewport(&self.gl);
                // }
                Event::KeyDown { keycode: Some(key_code), repeat: false, .. } => {
                    println!("KEY DOWN");
                    input.update_keyboard(key_code, true);
                    // self.key_map.insert(key_code, true);
                },
                Event::KeyUp { keycode: Some(key_code), repeat: false, .. } => {
                    input.update_keyboard(key_code, false);
                    // self.key_map.insert(key_code, false);
                },
                Event::MouseMotion { x, y, .. } => {
                    self.screen_mouse_position = Vector2::new(x, y);
                    input.update_screen_mouse(Vector2::new(x, y));
                    input.update_world_mouse(camera.convert_screen_to_world(Vector2::new(x, y)));
                }
                _ => {}
            }
        }
    }

    pub fn set_mouse_coord(&mut self, coord: Vector2<i32>) {
        self.screen_mouse_position = coord;
    }
}