mod builder;

use crate::dependency::Dependency;

pub use self::builder::ConfigBuilder;

use std::path::PathBuf;

pub struct Config {
    pub(crate) root_dir: PathBuf,
    pub(crate) protos: Vec<PathBuf>,

    pub(crate) dependencies: Vec<Dependency>,
    pub(crate) file_descriptors: bool,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}
