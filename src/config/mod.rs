mod apis;
mod incluster_config;
mod kube_config;
mod utils;

use failure::Error;
use hyper::client::HttpConnector;
use hyper::header::{Basic, Bearer};
use hyper::Client;
use hyper_tls::HttpsConnector;
use kubernetes::apis::configuration::Configuration;
use native_tls::Pkcs12;
use native_tls::TlsConnector;
use tokio_core::reactor::Handle;

use self::kube_config::KubeConfigLoader;

/// Returns a config includes authentication and cluster infomation from kubeconfig file.
///
/// # Example
/// ```no_run
/// use kubernetes::config;
///
/// let kubeconfig = config::load_kube_config()
///     .expect("failed to load kubeconfig");
/// ```
pub fn load_kube_config(
    threads: usize,
    handle: &Handle,
) -> Result<Configuration<HttpsConnector<HttpConnector>>, Error> {
    let kubeconfig = utils::kubeconfig_path()
        .or_else(utils::default_kube_path)
        .ok_or(format_err!("Unable to load kubeconfig"))?;

    let loader = KubeConfigLoader::load(kubeconfig)?;

    let mut tls = TlsConnector::builder()?;
    let p12 = loader.p12(" ")?;
    tls.identity(Pkcs12::from_der(&p12.to_der()?, " ")?)?;

    let ca = loader.ca()?;
    tls.add_root_certificate(ca)?;

    let (bearer, basic) = match (
        utils::data_or_file(&loader.user.token, &loader.user.token_file),
        (loader.user.username, loader.user.password),
    ) {
        (Ok(token), _) => (Some(Bearer { token: token }), None),
        (_, (Some(u), maybe_p)) => (
            None,
            Some(Basic {
                username: u,
                password: maybe_p,
            }),
        ),
        _ => (None, None),
    };

    let mut http = HttpConnector::new(threads, &handle);
    http.enforce_http(false);
    let client_builder = Client::configure().connector(HttpsConnector::from((http, tls.build()?)));

    Ok(Configuration {
        base_path: loader.cluster.server,
        client: client_builder.build(&handle),
        bearer: bearer,
        basic: basic,
    })
}

/// Returns a config which is used by clients within pods on kubernetes.
/// It will return an error if called from out of kubernetes cluster.
///
/// # Example
/// ```no_run
/// use kubernetes::config;
///
/// let kubeconfig = config::incluster_config()
///     .expect("failed to load incluster config");
/// ```
pub fn incluster_config(
    threads: usize,
    handle: &Handle,
) -> Result<Configuration<HttpsConnector<HttpConnector>>, Error> {
    let server = incluster_config::kube_server().ok_or(format_err!(
        "Unable to load incluster config, {} and {} must be defined",
        incluster_config::SERVICE_HOSTENV,
        incluster_config::SERVICE_PORTENV
    ))?;

    let mut tls = TlsConnector::builder()?;
    let ca = incluster_config::load_cert()?;
    tls.add_root_certificate(ca)?;

    let token = incluster_config::load_token()?;

    let mut http = HttpConnector::new(threads, &handle);
    http.enforce_http(false);
    let client_builder = Client::configure().connector(HttpsConnector::from((http, tls.build()?)));

    Ok(Configuration {
        base_path: server,
        client: client_builder.build(&handle),
        bearer: Some(Bearer { token: token }),
        basic: None,
    })
}
