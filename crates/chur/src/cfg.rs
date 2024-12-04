mod builder;

use crate::dependency::Dependency;

pub use self::builder::ConfigBuilder;

use std::path::PathBuf;


#[cfg(feature = "codegen")]
mod codegen_path;

#[cfg(feature = "codegen")]
pub use codegen_path::CodegenPath;

pub struct Config {
    pub(crate) root_dir: PathBuf,
    pub(crate) protos: Vec<PathBuf>,

    pub(crate) dependencies: Vec<Dependency>,
    pub(crate) file_descriptors: bool,

    #[cfg(feature = "codegen")]
    pub(crate) codegen: Option<CodegenPath>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}
