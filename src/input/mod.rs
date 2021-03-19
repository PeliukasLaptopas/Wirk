use std::collections::HashMap;
use sdl2::event::{EventPollIterator, Event};
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

pub struct InputManager {
    key_map: HashMap<Keycode, bool>
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            key_map: HashMap::new()
        }
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

    pub fn run(&self, epi: &mut EventPump) {
        for event in epi.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(key_code), repeat: false, .. } => {
                    println!("keycode: {}", key_code);
                }
                _ => {}
            }
        }
    }
}