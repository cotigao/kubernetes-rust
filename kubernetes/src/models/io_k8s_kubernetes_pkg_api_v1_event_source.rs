/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApiV1EventSource : EventSource contains information for an event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1EventSource {
  /// Component from which the event is generated.
  #[serde(rename = "component")]
  component: Option<String>,
  /// Node name on which the event is generated.
  #[serde(rename = "host")]
  host: Option<String>
}

impl IoK8sKubernetesPkgApiV1EventSource {
  /// EventSource contains information for an event.
  pub fn new() -> IoK8sKubernetesPkgApiV1EventSource {
    IoK8sKubernetesPkgApiV1EventSource {
      component: None,
      host: None
    }
  }

  pub fn set_component(&mut self, component: String) {
    self.component = Some(component);
  }

  pub fn with_component(mut self, component: String) -> IoK8sKubernetesPkgApiV1EventSource {
    self.component = Some(component);
    self
  }

  pub fn component(&self) -> Option<&String> {
    self.component.as_ref()
  }

  pub fn reset_component(&mut self) {
    self.component = None;
  }

  pub fn set_host(&mut self, host: String) {
    self.host = Some(host);
  }

  pub fn with_host(mut self, host: String) -> IoK8sKubernetesPkgApiV1EventSource {
    self.host = Some(host);
    self
  }

  pub fn host(&self) -> Option<&String> {
    self.host.as_ref()
  }

  pub fn reset_host(&mut self) {
    self.host = None;
  }

}



