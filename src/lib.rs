#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

extern crate base64;
extern crate dirs;
extern crate hyper;
extern crate hyper_tls;
extern crate kubernetes;
extern crate native_tls;
extern crate openssl;
extern crate serde;
extern crate serde_yaml;
extern crate tokio_core;

pub mod config;
