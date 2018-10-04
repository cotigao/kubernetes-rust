extern crate dirs;
extern crate kubernetes_rust;
#[macro_use]
extern crate failure;

use std::env;
use std::path::PathBuf;

use kubernetes_rust::Config;

fn main() {
    let mut p = dirs::home_dir().unwrap();
    p.push(".kube");
    p.push("config");
    let c = Config::load(p);
    println!("{:?}", c.unwrap());
}
