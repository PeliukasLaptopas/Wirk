use core::fmt;

pub struct ShaderError(pub String);

impl fmt::Debug for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShaderError")
            .field("error", &self.0)
            .finish()
    }
}