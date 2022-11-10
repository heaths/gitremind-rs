// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

mod config;
use std::path::PathBuf;

pub use config::Config;
use thiserror::Error;

pub fn find_config_dir() -> Result<Option<PathBuf>> {
    let cwd = std::env::current_dir()?;
    for path in cwd.ancestors() {
        let file_path = path.join(".gitremind");
        if file_path.exists() {
            return Ok(Some(file_path));
        }
    }

    Ok(None)
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to deserialize: {0}")]
    Deserialization(#[from] serde_yaml::Error),

    #[error("failed to read: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to write to terminal: {0}")]
    Terminal(#[from] term::Error),
}
