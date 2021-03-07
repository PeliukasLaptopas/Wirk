use std::io;
use std::io::Read;

#[derive(Debug)]
pub enum ResourceError {
    FailedToGetExePath,
    IoError(io::Error),
    FileContainsNil,
    LinkError
}
