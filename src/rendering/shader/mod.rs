pub mod program;
pub mod shader_utils;

use crate::rendering::errors::ShaderError;
use crate::resources::Resources;
use std::ffi::{CStr};
use gl::types::*;
use gl::*;
use crate::rendering::errors::ShaderError::{CanNotDetermineShaderTypeForResource, ErrorLoadingShaderFromResource, CompileError};
use crate::rendering::shader::shader_utils::create_whitespace_cstring_with_len;

pub struct Shader {
    gl: gl::Gl,
    id: GLuint,
}

impl Shader {
    fn shader_from_source(
        gl: &gl::Gl,
        source: &CStr, // modified
        kind: GLenum
    ) -> Result<Shader, String> {
        let id = unsafe { gl.CreateShader(kind) };

        unsafe {
            gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl.CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl.GetShaderiv(id, COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl.GetShaderiv(id, INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader { gl: gl.clone(), id })
    }


    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Shader, ShaderError> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER),
        ];

        let shader_kind = POSSIBLE_EXT.iter()
            .find(|&&(file_extension, _)| {
                name.ends_with(file_extension)
            })
            .map(|&(_, kind)| kind)
            .ok_or_else(|| CanNotDetermineShaderTypeForResource { name: name.to_string() })?;

        let source = res.load_cstring(name)
            .map_err(|e| ErrorLoadingShaderFromResource { name: name.to_string(), inner: e })?;

        Shader::shader_from_source(gl, &source, shader_kind).map_err(|message| CompileError { name: name.into(), message, })
    }

    pub fn create_vertex_shader(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::shader_from_source(
            gl,
            source,
            VERTEX_SHADER
        )
    }

    pub fn create_fragment_shader(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::shader_from_source(
            gl,
            source,
            FRAGMENT_SHADER
        )
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}
