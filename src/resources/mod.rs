pub mod resource_utils;

use crate::resources::resource_utils::resource_name_to_path;
use crate::resources::errors::ResourceError::*;
use crate::resources::errors::ResourceError;
use std::path::{PathBuf, Path};
use std::ffi::CString;
use std::{fs, ffi};
use std::fs::File;
use std::io::Read;

pub mod errors;

pub struct Resources {
    root_path: PathBuf
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

    pub fn load_cstring(&self, resource_name: &str) -> Result<CString, ResourceError> {
        println!("{}", resource_name_to_path(&self.root_path,resource_name).into_os_string().into_string().unwrap());

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