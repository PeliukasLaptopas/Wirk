use std::io;
use crate::resources::errors::ResourceError;
use std::path::{Path, PathBuf};

impl From<io::Error> for ResourceError {
    fn from(other: io::Error) -> Self {
        ResourceError::IoError(other)
    }
}

pub fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
