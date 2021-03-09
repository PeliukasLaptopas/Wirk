use crate::resources::errors::ResourceError;

#[derive(Debug, Fail)] // derive Fail, in addition to Debug
pub enum ShaderError {
    #[fail(display = "Can not determine shader type for resource {}", name)]
    CanNotDetermineShaderTypeForResource { name: String },
    #[fail(display = "Failed to load resource {}", name)]
    ErrorLoadingShaderFromResource { name: String, #[cause] inner: ResourceError },
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String },
}