mod apis;
mod loader;
mod utils;

use std::convert::From;

use failure::Error;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use kubernetes::apis::configuration::Configuration;
use native_tls;
use native_tls::TlsConnector;
use tokio_core::reactor::Handle;

use self::loader::KubeConfigLoader;

pub fn load_kube_config(
    threads: usize,
    handle: &Handle,
) -> Result<Configuration<HttpsConnector<HttpConnector>>, Error> {
    let kubeconfig = utils::kube_path()
        .or_else(utils::default_kube_path)
        .ok_or(format_err!("Unable to load config"))?;

    let loader = KubeConfigLoader::load(kubeconfig)?;
    let mut tls = TlsConnector::builder()?;

    let password = "";
    let p12 = loader.build_p12(password)?;
    tls.identity(native_tls::Pkcs12::from_der(&p12.to_der()?, password)?)?;

    let ca_pem = loader.build_ca_pem()?;
    tls.add_root_certificate(ca_pem)?;

    let mut http = HttpConnector::new(threads, &handle);
    http.enforce_http(false);

    let client = Client::configure()
        .connector(HttpsConnector::from((http, tls.build()?)))
        .build(&handle);

    let config = Configuration {
        base_path: loader.current_server().to_owned(),
        user_agent: Some("OpenAPI-Generator/v1.7.12/rust".to_owned()),
        client: client,
        basic_auth: None,
        oauth_access_token: None,
        api_key: None,
    };

    Ok(config)
}
