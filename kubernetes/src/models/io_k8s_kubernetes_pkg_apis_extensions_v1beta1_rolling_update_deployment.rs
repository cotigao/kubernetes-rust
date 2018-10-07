/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApisExtensionsV1beta1RollingUpdateDeployment : Spec to control the desired behavior of rolling update.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisExtensionsV1beta1RollingUpdateDeployment {
  #[serde(rename = "maxSurge")]
  max_surge: Option<String>,
  #[serde(rename = "maxUnavailable")]
  max_unavailable: Option<String>
}

impl IoK8sKubernetesPkgApisExtensionsV1beta1RollingUpdateDeployment {
  /// Spec to control the desired behavior of rolling update.
  pub fn new() -> IoK8sKubernetesPkgApisExtensionsV1beta1RollingUpdateDeployment {
    IoK8sKubernetesPkgApisExtensionsV1beta1RollingUpdateDeployment {
      max_surge: None,
      max_unavailable: None
    }
  }

  pub fn set_max_surge(&mut self, max_surge: String) {
    self.max_surge = Some(max_surge);
  }

  pub fn with_max_surge(mut self, max_surge: String) -> IoK8sKubernetesPkgApisExtensionsV1beta1RollingUpdateDeployment {
    self.max_surge = Some(max_surge);
    self
  }

  pub fn max_surge(&self) -> Option<&String> {
    self.max_surge.as_ref()
  }

  pub fn reset_max_surge(&mut self) {
    self.max_surge = None;
  }

  pub fn set_max_unavailable(&mut self, max_unavailable: String) {
    self.max_unavailable = Some(max_unavailable);
  }

  pub fn with_max_unavailable(mut self, max_unavailable: String) -> IoK8sKubernetesPkgApisExtensionsV1beta1RollingUpdateDeployment {
    self.max_unavailable = Some(max_unavailable);
    self
  }

  pub fn max_unavailable(&self) -> Option<&String> {
    self.max_unavailable.as_ref()
  }

  pub fn reset_max_unavailable(&mut self) {
    self.max_unavailable = None;
  }

}



