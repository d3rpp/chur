use std::{
    env,
    path::PathBuf,
    process::Command,
    str::{from_utf8, FromStr},
};

const CHUR_DIR: &str = "chur";

lazy_static::lazy_static! {
    /// Setup I found tends to work for locating the workspace root.
    pub(crate) static ref ROOT_MANIFEST_DIR: PathBuf = {
        let cargo_command = env::var("CARGO")
            .expect("Unable to locate Cargo binary");

        let manifest_file_command = Command::new(cargo_command)
            .arg("locate-project")
            .arg("--workspace")
            .arg("--message-format")
            .arg("plain")
            .arg("--quiet")
            .output();

        match manifest_file_command {
            Ok(output) => {
                if !output.status.success() {
                    panic!("Failed to locate workspace root - {}", from_utf8(&output.stderr).expect("Failed to parse cargo error"));
                }

                let root_path = from_utf8(&output.stdout)
                    .expect("Unable to read workspace root");

                PathBuf::from_str(root_path)
                    .expect("Unable to parse workspace root")
                    .parent()
                    .expect("Located workspace root is in root of file system")
                    .to_path_buf()
            },


            Err(e) => {
                panic!("Failed to spawn cargo process - {}", e.to_string());
            }
        }
    };

    /// This whole library breaks a few standards but does use the target folder for
    /// caching clones, as such we need to be able to create a folder in the `target` dir.
    ///
    /// > Since it generates sources `<workspace root>/target/chur` will work.
    pub(crate) static ref CHUR_ROOT_DIR: PathBuf = ROOT_MANIFEST_DIR
        .join("target")
        .join(CHUR_DIR);

    /// Storage place for manifest file.
    pub(crate) static ref MANIFEST_FILE_NAME: PathBuf = CHUR_ROOT_DIR
        .join("manifest.ron");

    /// Dependency Cache Directory.
    pub(crate) static ref DEPENDENCY_CACHE_DIR: PathBuf = CHUR_ROOT_DIR
        .join("cached_depedencies");

    /// Dependency Include Directory.
    pub(crate) static ref DEPENDENCY_INCLUDE_DIR: PathBuf = CHUR_ROOT_DIR
        .join("include");

    /// Generated Sources Directory.
    pub(crate) static ref GENERATED_SOURCES_DIR: PathBuf = CHUR_ROOT_DIR
        .join("generated"); 
}
