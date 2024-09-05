use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::util::file_perm::secure_file;
use config::{Config, Environment, File};

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    pub store: GlobalConfigStore,
    pub paths_relative_to: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalConfigStore {
    name: String,
}

pub fn log_config(path: &Path, secure: bool) -> Result<GlobalConfig> {
    // 校验文件是否安全
    if secure {
        secure_file(path).context("Could not secure config")?;
    }
    // 读取文件
    let mut store: serde_yaml::Value = Config::builder()
        .add_source(File::from(path))
        .add_source(Environment::with_prefix("YP"))
        .build()
        .context("Could not load config")?
        .try_deserialize()
        .context("Could not parse YAML")?;

    let store: GlobalConfigStore =
        serde_yaml::from_value(store).context("Could not load config")?;
    
    let config = GlobalConfig{
        store,
        paths_relative_to: path.parent().context("FS root reached")?.to_path_buf(),
    };
    Ok(config)
}
