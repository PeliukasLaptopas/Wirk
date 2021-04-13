use std::ffi::c_void;
use sdl2::pixels::Color;

pub struct Texture {
    pub id: gl::types::GLuint,
    pub width: u32,
    pub height: u32,
} //todo no pub?
