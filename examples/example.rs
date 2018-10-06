extern crate dirs;
extern crate kubernetes_rust;

use kubernetes_rust::Config;

fn main() {
    let mut p = dirs::home_dir().unwrap();
    p.push(".kube");
    p.push("config");
    let c = Config::load(p);
    println!("{:?}", c.unwrap());
}
