pub mod engine_error;
pub mod resources;
pub mod rendering;

#[macro_use] extern crate gl_derive;
#[macro_use] extern crate failure;

extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;
extern crate nalgebra;

use nalgebra as na;
use std::rc::Rc;
use std::path::Path;
use crate::resources::Resources;
use failure::err_msg;
use crate::rendering::shader::program::Program;
use crate::rendering::{vertex, triangle};
use crate::rendering::vertex::Vertex;
use crate::rendering::shader::buffer;
use crate::rendering::shader::buffer::{Buffer, BufferTypeArray, VertexArray};
use crate::rendering::shader::viewport::Viewport;
use crate::rendering::shader::color_buffer::ColorBuffer;
use na::*;

pub fn open_window() -> Result<(), failure::Error> {
    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

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

    let res = Resources::from_relative_path(Path::new("assets")).unwrap();
    let shader_program = Program::from_res(&gl, &res, "shaders/triangle").unwrap();

    let vertices: Vec<Vertex> = vec![
        Vertex {
            pos: (0.5, -0.5, 0.0).into(),
            color: (1.0, 0.0, 0.0, 1.0).into()
        },
        Vertex {
            pos: (-0.5, -0.5, 0.0).into(),
            color: (0.0, 1.0, 0.0, 1.0).into()
        },
        Vertex {
            pos: (0.0,  0.5, 0.0).into(),
            color: (0.0, 0.0, 1.0, 1.0).into()
        }
    ];

    // set up vertex buffer object
    let vbo: Buffer<BufferTypeArray> = buffer::ArrayBuffer::new(&gl);
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    // set up vertex array object
    let vao: VertexArray = buffer::VertexArray::new(&gl);
    vao.bind();
    vbo.bind();
    Vertex::vertex_attrib_pointers(&gl);
    vbo.unbind();
    vao.unbind();

    let mut viewport = Viewport::for_window(900, 700);

    let color_buffer = ColorBuffer::from_color(Vector3::new(0.3, 0.3, 0.5));

    let triangle = triangle::Triangle::new(&Vector2::new(-1.0, -1.0), &Vector2::new(1.0, 1.0), &res, &gl)?;

    viewport.use_viewport(&gl);
    color_buffer.clear_color(&gl);

    // main loop
    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    'main: loop {
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
        triangle.render(&gl);

        window.gl_swap_window();
    }

    Ok(())
}