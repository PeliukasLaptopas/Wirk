pub mod texture_cache;
pub mod resource_utils;

use crate::resources::resource_utils::resource_name_to_path;
use crate::resources::errors::ResourceError::*;
use crate::resources::errors::ResourceError;
use std::path::{PathBuf, Path};
use std::ffi::CString;
use std::{fs};
use std::io::Read;
use image::io::Reader as ImageReader;

pub mod errors;

pub struct Resources {
    pub root_path: PathBuf //todo not pub
}

impl Resources {
    pub fn from_relative_path(rel_path: &Path) -> Result<Resources, ResourceError> {

        let exe_file_name = ::std::env::current_exe()
            .map_err(|_| FailedToGetExePath)?;

        let exe_path = exe_file_name.parent()
            .ok_or(FailedToGetExePath)?;

        Ok(Resources {
            root_path: exe_path.join(rel_path)
        })
    }

    /*pub fn load_png(&self, resource_name: &str) -> Result<CString, ResourceError> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path,resource_name)
        )?;

        let img = ImageReader::open("myimage.png")?.decode()?;

        let opened_img = ImageReader::open("myimage.png").map_err(|e| e);

        // allocate buffer of the same size as file
        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut buffer)?;

        // check for nul byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(FileContainsNil);
        }

        Ok(unsafe { CString::from_vec_unchecked(buffer) })
    }*/


    pub fn load_cstring(&self, resource_name: &str) -> Result<CString, ResourceError> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path,resource_name)
        )?;

        // allocate buffer of the same size as file
        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut buffer)?;

        // check for nul byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(FileContainsNil);
        }

        Ok(unsafe { CString::from_vec_unchecked(buffer) })
    }
}