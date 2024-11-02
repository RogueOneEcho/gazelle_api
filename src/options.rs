use crate::Action::CreateClient;
use crate::InnerError::{Yaml, IO};
use crate::{Error, GazelleClient, GazelleClientFactory};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct GazelleOptions {
    pub api_key: String,
    pub api_url: String,
}

impl GazelleOptions {
    fn from_file(path: &Path) -> Result<Self, Error> {
        let file = File::open(path).map_err(|e| Error {
            action: CreateClient,
            message: None,
            status_code: None,
            inner: Some(IO(e)),
        })?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).map_err(|e| Error {
            action: CreateClient,
            message: None,
            status_code: None,
            inner: Some(Yaml(e)),
        })
    }
}

pub fn get_test_client() -> Result<GazelleClient, Error> {
    let options = GazelleOptions::from_file(&PathBuf::from("config.yml"))?;
    let factory = GazelleClientFactory {
        key: options.api_key,
        url: options.api_url,
        user_agent: "gazelle_api".to_owned(),
    };
    Ok(factory.create())
}
