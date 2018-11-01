extern crate failure;
extern crate futures;
extern crate kubernetes;
extern crate rust_base;
extern crate tokio_core;

use futures::Future;
use tokio_core::reactor::Core;

use kubernetes::apis::client::APIClient;
use rust_base::config;

fn main() {
    let mut event_loop = Core::new().expect("failed to initialize core");
    let kubeconfig =
        config::load_kube_config(1, &event_loop.handle()).expect("failed to load kubeconfig");
    let client = APIClient::new(kubeconfig);
    let work = client.core_api().get_api_versions().inspect(|v| {
        println!("{:?}", v);
    });
    event_loop.run(work).expect("failed to run core");
}
