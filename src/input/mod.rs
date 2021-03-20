use std::collections::HashMap;
use sdl2::event::{EventPollIterator, Event};
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use nalgebra::Vector2;

pub struct InputManager {
    key_map: HashMap<Keycode, bool>,
    pub screen_mouse_position: Vector2<i32>,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            key_map: HashMap::new(),
            screen_mouse_position: Vector2::new(0, 0),
        }
    }

    pub fn set_mouse_coord(&mut self, coord: Vector2<i32>) {
        self.screen_mouse_position = coord;
    }

    pub fn press_key(&mut self, key_id: &Keycode) {
        self.key_map.insert(*key_id, true);
    }

    pub fn release_key(&mut self, key_id: &Keycode) {
        self.key_map.insert(*key_id, false);
    }

    pub fn is_key_pressed(&self, key_id: &Keycode) -> bool {
        if self.key_map.contains_key(key_id) {
            self.key_map[key_id]
        } else {
            false
        }
    }
}