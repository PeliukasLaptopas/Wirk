pub mod engine_error;
pub mod resources;
pub mod rendering;

#[macro_use] extern crate gl_derive;
#[macro_use] extern crate failure;

extern crate sdl2;
extern crate gl;

use std::rc::Rc;
use std::path::Path;
use crate::resources::Resources;
use failure::err_msg;
use crate::rendering::shader::program::Program;
use crate::rendering::vertex;
use crate::rendering::vertex::Vertex;

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
        Vertex { pos: (0.5, -0.5, 0.0).into(),  color: (1.0, 0.0, 0.0).into() }, // bottom right
        Vertex { pos: (-0.5, -0.5, 0.0).into(), color: (0.0, 1.0, 0.0).into() }, // bottom left
        Vertex { pos: (0.0,  0.5, 0.0).into(),  color: (0.0, 0.0, 1.0).into() }  // top
    ];

    let mut vbo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;

    unsafe {
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );

        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

        Vertex::vertex_attrib_pointers(&gl);

        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl.Viewport(0, 0, 900, 700); // set viewport
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.use_this();

        unsafe {
            gl.BindVertexArray(vao);
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }

        window.gl_swap_window();
        // render window contents here
    }

    Ok(())
}