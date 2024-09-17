use std::fmt::Display;

use crate::{defined_constants::ROOT_MANIFEST_DIR, dependency::Dependency};

use super::Config;
use crate::error::ChurResult;

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    root_dir: String,

    dependencies: Vec<Dependency>,

    protos: Vec<String>,
    file_descriptors: bool,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Root Directory of the location of your `.proto` files.
    ///
    /// This is a folder relative to the workspace directory.
    pub fn root_dir(mut self, root_dir: impl Display) -> Self {
        self.root_dir = root_dir.to_string();
        self
    }

    /// Define the dependencies of this project, refer to [Dependency](crate::Dependency)
    /// for documentation.
    ///
    /// Can be called multiple times for multiple dependencies.
    pub fn dependency(mut self, dependency: Dependency) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Define the proto files for compilation.
    ///
    /// For a gRPC setup this would only require you to references the wanted services,
    /// dependencies of those services will be imported.
    pub fn protos(mut self, protos: impl IntoIterator<Item = impl Display>) -> Self {
        let mut protos_as_strings = protos
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        self.protos.append(&mut protos_as_strings);

        self
    }

    pub fn file_descriptors(mut self, file_descriptors: bool) -> Self {
        self.file_descriptors = file_descriptors;
        self
    }

    /// Build the [ConfigBuilder] into a [Config]
    ///
    /// This changes the directories used to be absolute.
    pub fn build(self) -> ChurResult<Config> {
        let root_dir = ROOT_MANIFEST_DIR.join(self.root_dir);

        let protos = self
            .protos
            .into_iter()
            .map(|dir| root_dir.join(dir))
            .collect::<Vec<_>>();

        Ok(Config {
            root_dir,
            protos,
            dependencies: self.dependencies,
            file_descriptors: self.file_descriptors,
        })
    }
}
