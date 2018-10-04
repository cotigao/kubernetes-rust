#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate serde;
extern crate serde_yaml;
extern crate url;
extern crate url_serde;

extern crate kubernetes;

pub mod config;

pub use config::Config;
