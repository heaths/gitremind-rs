// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use crate::Result;
use serde::Deserialize;
use std::io;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    timeout: Option<u32>,
    message: String,
}

impl Config {
    pub fn parse<R>(reader: R) -> Result<Option<Config>>
    where
        R: io::Read,
    {
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn timeout(&self) -> u32 {
        self.timeout.unwrap_or(5)
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_default() {
        let config = Config::default();
        assert_eq!(5, config.timeout());
        assert_eq!("", config.message());
    }

    #[test]
    fn config_parse() {
        let yaml = r"
        timeout: 10
        message: test
        "
        .as_bytes();

        let config = Config::parse(yaml)
            .expect("failed to parse config")
            .expect("expected content");
        assert_eq!(10, config.timeout());
        assert_eq!("test", config.message());
    }

    #[test]
    fn config_error() {
        let yaml = r"
        message:
        - A
        - B
        "
        .as_bytes();

        let err = Config::parse(yaml).unwrap_err();
        assert!(format!("{}", err).starts_with("failed to deserialize: message"));
    }
}
