use std::fmt::Display;

#[cfg(feature = "codegen")]
use std::path::PathBuf;

use crate::{defined_constants::ROOT_MANIFEST_DIR, dependency::Dependency};

use super::Config;
use crate::error::ChurResult;

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    root_dir: String,

    dependencies: Vec<Dependency>,

    protos: Vec<String>,
    file_descriptors: bool,

    #[cfg(feature = "codegen")]
    codegen: Option<String>,
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

    /// Generate file descriptors
    pub fn file_descriptors(mut self, file_descriptors: bool) -> Self {
        self.file_descriptors = file_descriptors;
        self
    }

    #[cfg(feature = "codegen")]
    /// Instead of just making the manifest, forcing to use [`chur::include_tree`][include_tree]
    /// just dump the code into a file at the provided path.
    ///
    /// This works better with rust-analyzer.
    ///
    /// Provided path should be relative to the workspace `Cargo.toml`.
    ///
    /// [include_tree]: https://docs.rs/chur/latest/chur/macro.include_tree.html
    pub fn codegen(mut self, codegen_path: impl ToString) -> Self {
        self.codegen = Some(codegen_path.to_string());
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
            
            #[cfg(feature = "codegen")]
            codegen: self.codegen.map(PathBuf::from),
        })
    }
}
