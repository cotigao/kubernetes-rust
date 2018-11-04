extern crate failure;
extern crate futures;
extern crate k8s;
extern crate kubernetes;
extern crate tokio_core;

use futures::Future;
use tokio_core::reactor::Core;

use k8s::apis::client::APIClient;
use kubernetes::config;

fn main() {
    let mut event_loop = Core::new().expect("failed to initialize core");
    let kubeconfig =
        config::load_kube_config(1, &event_loop.handle()).expect("failed to load kubeconfig");
    let client = APIClient::new(kubeconfig);
    let work = client
        .core_v1_api()
        .list_namespaced_pod("kube-system", true, "", "", "", "", 1, "", 1, false)
        .inspect(|v| {
            println!("{:?}", v);
        });
    event_loop.run(work).expect("failed to run core");
}
