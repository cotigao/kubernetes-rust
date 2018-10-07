/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ServiceReference : ServiceReference holds a reference to Service.legacy.k8s.io

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ServiceReference {
  /// Name is the name of the service
  #[serde(rename = "name")]
  name: Option<String>,
  /// Namespace is the namespace of the service
  #[serde(rename = "namespace")]
  namespace: Option<String>
}

impl IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ServiceReference {
  /// ServiceReference holds a reference to Service.legacy.k8s.io
  pub fn new() -> IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ServiceReference {
    IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ServiceReference {
      name: None,
      namespace: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ServiceReference {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_namespace(&mut self, namespace: String) {
    self.namespace = Some(namespace);
  }

  pub fn with_namespace(mut self, namespace: String) -> IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ServiceReference {
    self.namespace = Some(namespace);
    self
  }

  pub fn namespace(&self) -> Option<&String> {
    self.namespace.as_ref()
  }

  pub fn reset_namespace(&mut self) {
    self.namespace = None;
  }

}



