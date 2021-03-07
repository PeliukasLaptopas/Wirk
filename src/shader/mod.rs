pub mod program;
pub mod errors;
pub mod shader_utils;

use crate::shader::shader_utils::create_whitespace_cstring_with_len;
use crate::shader::errors::ShaderError;
use crate::resources::Resources;
use std::ffi::{CStr};
use gl::types::*;
use gl::*;

pub struct Shader {
    gl: gl::Gl,
    id: GLuint,
}

impl Shader {
    fn shader_from_source(
        gl: &gl::Gl,
        source: &CStr, // modified
        kind: GLenum
    ) -> Result<Shader, ShaderError> {
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

            return Err(ShaderError(error.to_string_lossy().into_owned()));
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
            .ok_or_else(|| ShaderError(format!("Can not determine shader type for resource {}", name)))?;

        let source = res.load_cstring(name)
            .map_err(|e| ShaderError(format!("Error loading resource {}: {:?}", name, e)))?;

        Shader::shader_from_source(gl, &source, shader_kind)
    }

    pub fn create_vertex_shader(gl: &gl::Gl, source: &CStr) -> Result<Shader, ShaderError> {
        Shader::shader_from_source(
            gl,
            source,
            VERTEX_SHADER
        )
    }

    pub fn create_fragment_shader(gl: &gl::Gl, source: &CStr) -> Result<Shader, ShaderError> {
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
