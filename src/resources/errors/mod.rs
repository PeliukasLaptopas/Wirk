use std::io;

#[derive(Debug, Fail)]
pub enum ResourceError {
    #[fail(display = "Failed get executable path")]
    FailedToGetExePath,
    #[fail(display = "I/O error")]
    IoError(#[cause] io::Error),
    #[fail(display = "Failed to read CString from file that contains 0")]
    FileContainsNil,
}
