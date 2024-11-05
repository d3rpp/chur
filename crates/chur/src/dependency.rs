use std::fmt::Display;
use std::fs;
use std::io::{Read, Write};

use sha::sha256::Sha256;
use sha::utils::DigestExt;

use crate::defined_constants::{DEPENDENCY_CACHE_DIR, DEPENDENCY_INCLUDE_DIR};
use crate::error::{ChurError, ChurResult};

use archiver_rs::{Archive, Compressed};

#[derive(Debug)]
pub enum DependencyFormat {
    Gzip,
}

#[derive(Debug)]
pub struct Dependency {
    pub(crate) url: String,
    pub(crate) format: DependencyFormat,
    pub(crate) subdir: Option<String>,
}

impl Dependency {
    pub fn new(
        url: impl Display,
        format: DependencyFormat,
        subdir: impl Into<Option<String>>,
    ) -> Self {
        Self {
            url: url.to_string(),
            format,
            subdir: subdir.into(),
        }
    }

    pub fn tarball(
        url: impl Display,
        format: DependencyFormat,
        subdir: impl Into<Option<String>>,
    ) -> Self {
        Self {
            url: url.to_string(),
            format,
            subdir: subdir.into(),
        }
    }

    pub fn github(repo: impl Display, branch_or_hash: impl Into<Option<String>>) -> Self {
        let branch_opt: Option<String> = branch_or_hash.into();
        let branch_or_hash_unwrapped = branch_opt.unwrap_or("main".to_string());

        let repo_string = repo.to_string();
        let mut repo_split = repo_string.split("/");

        let _user = repo_split.next().unwrap();
        let repo = repo_split.next().unwrap();

        // the subdir of a github tarball depends on the repos name and the commit hash
        //
        // e.g. this would contain a subdir called chur-<branch_or_hash> which
        // has the repo in it.
        Self::new(
            format!("https://github.com/{repo_string}/archive/{branch_or_hash_unwrapped}.tar.gz"),
            DependencyFormat::Gzip,
            format!("{repo}-{branch_or_hash_unwrapped}"),
        )
    }

    pub(crate) fn fetch(&self) -> ChurResult<String> {
        match self.format {
            DependencyFormat::Gzip => self.fetch_gzip_tar(),
        }
    }

    pub(crate) fn fetch_gzip_tar(&self) -> ChurResult<String> {
        let agent = ureq::Agent::new();

        let response = agent.get(&self.url).call()?;
        if response.status() != 200 {
            ChurError::Dependency(format!(
                "Dependency with URL \"{}\" returns a result of {}",
                self.url,
                response.status()
            ));
        }

        // 2MB seems like a decent default limit.
        let response_len = if let Some(header) = response.header("Content-Length") {
            header.parse().unwrap_or(2_000_000)
        } else {
            2_000_000
        };

        let mut buf = Vec::with_capacity(response_len);
        response
            .into_reader()
            .take(response_len as u64)
            .read_to_end(&mut buf)?;

        let mut hasher = Sha256::default();
        hasher.write_all(&buf)?;

        let hash = hasher.to_hex();

        let mut archive = archiver_rs::Gzip::new(buf.as_slice())?;
        fs::create_dir_all(DEPENDENCY_CACHE_DIR.as_path())?;
        let tarball_file = DEPENDENCY_CACHE_DIR.join(&format!("tarball_{}", &hash));
        archive.decompress(&tarball_file)?;
        drop(archive);

        let mut tarball = archiver_rs::Tar::open(&tarball_file)?;
        let unpacked_dir = DEPENDENCY_INCLUDE_DIR.join(&hash);
        fs::create_dir_all(&unpacked_dir)?;
        tarball.extract(&unpacked_dir)?;

        Ok(hash)
    }
}
