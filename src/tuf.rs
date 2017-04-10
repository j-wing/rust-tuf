use json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;
use std::path::Path;
use url::Url;

use error::TufError;
use metadata::{Role, RoleType, Root, Metadata, RootMetadata};

pub struct Tuf {
    url: Url,
    local_path: Box<Path>,
    root: RootMetadata,
}

impl Tuf {
    pub fn new(config: Config) -> Result<Self, TufError> {
        let root = Self::read_metadata::<Root, RootMetadata>(&config.local_path)?;

        Ok(Tuf {
            url: config.url,
            local_path: config.local_path,
            root: root,
        })
    }

    fn read_metadata<R: RoleType, M: Metadata<R>>(local_path: &Path) -> Result<M, TufError> {
        Self::read_meta_prefix(local_path, "")
    }

    fn read_meta_num<R: RoleType, M: Metadata<R>>(local_path: &Path, num: i32) -> Result<M, TufError> {
        Self::read_meta_prefix(local_path, &format!("{}.", num))
    }

    fn read_meta_hash<R: RoleType, M: Metadata<R>>(local_path: &Path, hash: &str) -> Result<M, TufError> {
        Self::read_meta_prefix(local_path, &format!("{}.", hash))
    }

    fn read_meta_prefix<R: RoleType, M: Metadata<R>>(local_path: &Path, prefix: &str) -> Result<M, TufError> {
        let path = local_path.join(format!("{}{}.json", prefix, R::role()));        
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let jsn = json::from_slice(&buf)?;
        let safe_bytes = Self::verify_meta(jsn)?;
        Ok(json::from_slice(&safe_bytes)?)
    }

    /// Consumes the JSON because we only care about parsing the output. Bytes are only trusted
    /// after they are verified. We do this to mitigate exploits that rely on different JSON
    /// parsers parsing JSON in different ways.
    fn verify_meta(jsn: json::Value) -> Result<Vec<u8>, TufError> {
        unimplemented!() // TODO
    }

    // TODO real return type
    pub fn list_targets() -> Vec<String> {
        unimplemented!() // TODO
    }

    // TODO real input type
    pub fn fetch_target(target: String) -> Result<Box<Path>, TufError> {
        unimplemented!() // TODO
    }
}

pub struct Config {
    url: Url,
    local_path: Box<Path>,
}

impl Config {
    pub fn build() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}


pub struct ConfigBuilder {
    url: Option<Url>,
    local_path: Option<Box<Path>>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder {
            url: None,
            local_path: None,
        }
    }

    pub fn url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    pub fn local_path(mut self, local_path: Box<Path>) -> Self {
        self.local_path = Some(local_path);
        self
    }

    pub fn finish(self) -> Result<Config, TufError> {
        let url = self.url.ok_or(TufError::InvalidConfig("Repository URL was not set".to_string()))?;
        let local_path = self.local_path.ok_or(TufError::InvalidConfig("Local path was not set".to_string()))?;

        // TODO error if path is not fully owned by the current user
        // TODO create path if not exists

        Ok(Config {
            url: url,
            local_path: local_path,
        })
    }
}