use std::{collections::HashMap, fmt::Display, fs, io::Read};

use crate::{
    defined_constants::MANIFEST_FILE_NAME,
    error::{ChurError, ChurResult},
};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub(crate) struct Manifest {
    #[serde(default)]
    depenedencies: HashMap<String, String>,
}

impl Manifest {
    pub(crate) fn load() -> ChurResult<Self> {
        let mut manifest_file = fs::OpenOptions::new()
            .read(true)
            .open(MANIFEST_FILE_NAME.as_path())?;
        let mut buf = String::new();

        manifest_file.read_to_string(&mut buf)?;

        ron::from_str::<Self>(&buf).map_err(ChurError::from)
    }

    pub(crate) fn save(&self) -> ChurResult<()> {
        fs::create_dir_all(MANIFEST_FILE_NAME.parent().unwrap())?;
        let manifest_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(MANIFEST_FILE_NAME.as_path())?;

        let cfg: ron::ser::PrettyConfig = ron::ser::PrettyConfig::default();
        ron::Options::default().to_writer_pretty(manifest_file, self, cfg)?;

        Ok(())
    }

    pub(crate) fn get_cached_dep_from_url(&self, url: impl Display) -> Option<String> {
        self.depenedencies.get(&url.to_string()).cloned()
    }

    pub(crate) fn register_cached_manifest(&mut self, url: impl Display, hash: impl Display) {
        self.depenedencies.insert(url.to_string(), hash.to_string());
    }
}
