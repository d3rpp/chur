use std::fs;

use crate::{defined_constants::{DEPENDENCY_INCLUDE_DIR, GENERATED_SOURCES_DIR}, error::ChurResult, manifest::Manifest, Config};


pub fn execute(cfg: Config) -> ChurResult<()> {
    let mut manifest = Manifest::load().unwrap_or_default();

    // include dir relative to DEPENDENCY_INCLUDE_DIR
    let mut additional_include_dirs = vec![];

    for dep in cfg.dependencies {
        if let Some(hash) = manifest.get_cached_dep_from_url(&dep.url) {
            additional_include_dirs.push(format!("{hash}/{}", dep.subdir.unwrap_or_default()));
        } else {
            let hash = dep.fetch()?;
            manifest.register_cached_manifest(dep.url, &hash);
            additional_include_dirs.push(format!("{hash}/{}", dep.subdir.unwrap_or_default()));
        }
    }

    manifest.save()?;

    let mut include_dirs = vec![cfg.root_dir];

    fs::create_dir_all(GENERATED_SOURCES_DIR.as_path())?;

    let builder = tonic_build::configure()
        .out_dir(GENERATED_SOURCES_DIR.as_path());

    for dir in additional_include_dirs {
        include_dirs.push(DEPENDENCY_INCLUDE_DIR.join(dir));
    }

    builder.compile(
        &cfg.protos, 
        &include_dirs
    )?;

    println!("cargo::rustc-env=__CHUR_DIR={}", GENERATED_SOURCES_DIR.display());

    Ok(())
}