/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1WebhookClientConfig : WebhookClientConfig contains the information to make a TLS connection with the webhook

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1WebhookClientConfig {
  /// `caBundle` is a PEM encoded CA bundle which will be used to validate the webhook's server certificate. Required.
  #[serde(rename = "caBundle")]
  ca_bundle: String,
  /// `service` is a reference to the service for this webhook. Either `service` or `url` must be specified.  If the webhook is running within the cluster, then you should use `service`.  Port 443 will be used if it is open, otherwise it is an error.
  #[serde(rename = "service")]
  service: Option<::models::AdmissionregistrationV1beta1ServiceReference>,
  /// `url` gives the location of the webhook, in standard URL form (`[scheme://]host:port/path`). Exactly one of `url` or `service` must be specified.  The `host` should not refer to a service running in the cluster; use the `service` field instead. The host might be resolved via external DNS in some apiservers (e.g., `kube-apiserver` cannot resolve in-cluster DNS as that would be a layering violation). `host` may also be an IP address.  Please note that using `localhost` or `127.0.0.1` as a `host` is risky unless you take great care to run this webhook on all hosts which run an apiserver which might need to make calls to this webhook. Such installs are likely to be non-portable, i.e., not easy to turn up in a new cluster.  The scheme must be \"https\"; the URL must begin with \"https://\".  A path is optional, and if present may be any string permissible in a URL. You may use the path to pass an arbitrary string to the webhook, for example, a cluster identifier.  Attempting to use a user or basic auth e.g. \"user:password@\" is not allowed. Fragments (\"#...\") and query parameters (\"?...\") are not allowed, either.
  #[serde(rename = "url")]
  url: Option<String>
}

impl V1beta1WebhookClientConfig {
  /// WebhookClientConfig contains the information to make a TLS connection with the webhook
  pub fn new(ca_bundle: String) -> V1beta1WebhookClientConfig {
    V1beta1WebhookClientConfig {
      ca_bundle: ca_bundle,
      service: None,
      url: None
    }
  }

  pub fn set_ca_bundle(&mut self, ca_bundle: String) {
    self.ca_bundle = ca_bundle;
  }

  pub fn with_ca_bundle(mut self, ca_bundle: String) -> V1beta1WebhookClientConfig {
    self.ca_bundle = ca_bundle;
    self
  }

  pub fn ca_bundle(&self) -> &String {
    &self.ca_bundle
  }


  pub fn set_service(&mut self, service: ::models::AdmissionregistrationV1beta1ServiceReference) {
    self.service = Some(service);
  }

  pub fn with_service(mut self, service: ::models::AdmissionregistrationV1beta1ServiceReference) -> V1beta1WebhookClientConfig {
    self.service = Some(service);
    self
  }

  pub fn service(&self) -> Option<&::models::AdmissionregistrationV1beta1ServiceReference> {
    self.service.as_ref()
  }

  pub fn reset_service(&mut self) {
    self.service = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> V1beta1WebhookClientConfig {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

}


