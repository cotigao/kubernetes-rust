use std::fs::File;
use std::path::Path;

use failure::Error;

use serde_yaml;
use url::Url;
use url_serde;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub kind: Option<String>,
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,
    pub preferences: Option<Preferences>,
    pub clusters: Vec<NamedCluster>,
    #[serde(rename = "users")]
    pub auth_infos: Vec<NamedAuthInfo>,
    pub contexts: Vec<NamedContext>,
    #[serde(rename = "current-context")]
    pub current_context: String,
    pub extensions: Option<Vec<NamedExtension>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preferences {
    pub colors: Option<bool>,
    pub extensions: Option<Vec<NamedExtension>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedExtension {
    pub name: String,
    pub extension: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedCluster {
    pub name: String,
    pub cluster: Cluster,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(with = "url_serde")]
    pub server: Url,
    #[serde(rename = "insecure-skip-tls-verify")]
    pub insecure_skip_tls_verify: Option<bool>,
    #[serde(rename = "certificate-authority")]
    pub certificate_authority: Option<String>,
    #[serde(rename = "certificate-authority-data")]
    pub certificate_authority_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedAuthInfo {
    pub name: String,
    #[serde(rename = "user")]
    pub auth_info: AuthInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub username: Option<String>,
    pub password: Option<String>,

    pub token: Option<String>,
    #[serde(rename = "tokenFile")]
    pub token_file: Option<String>,

    #[serde(rename = "client-certificate")]
    pub client_certificate: Option<String>,
    #[serde(rename = "client-certificate-data")]
    pub client_certificate_data: Option<String>,

    #[serde(rename = "client-key")]
    pub client_key: Option<String>,
    #[serde(rename = "client-key-data")]
    pub client_key_data: Option<String>,

    #[serde(rename = "as")]
    pub impersonate: Option<String>,
    #[serde(rename = "as-groups")]
    pub impersonate_groups: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedContext {
    pub name: String,
    pub context: Context,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub cluster: String,
    pub user: String,
    pub namespace: Option<String>,
    pub extensions: Option<Vec<NamedExtension>>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let f = File::open(path)?;
        let config = serde_yaml::from_reader(f)?;
        Ok(config)
    }
}