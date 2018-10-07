/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApiV1PersistentVolumeStatus : PersistentVolumeStatus is the current status of a persistent volume.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1PersistentVolumeStatus {
  /// A human-readable message indicating details about why the volume is in this state.
  #[serde(rename = "message")]
  message: Option<String>,
  /// Phase indicates if a volume is available, bound to a claim, or released by a claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase
  #[serde(rename = "phase")]
  phase: Option<String>,
  /// Reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.
  #[serde(rename = "reason")]
  reason: Option<String>
}

impl IoK8sKubernetesPkgApiV1PersistentVolumeStatus {
  /// PersistentVolumeStatus is the current status of a persistent volume.
  pub fn new() -> IoK8sKubernetesPkgApiV1PersistentVolumeStatus {
    IoK8sKubernetesPkgApiV1PersistentVolumeStatus {
      message: None,
      phase: None,
      reason: None
    }
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> IoK8sKubernetesPkgApiV1PersistentVolumeStatus {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_phase(&mut self, phase: String) {
    self.phase = Some(phase);
  }

  pub fn with_phase(mut self, phase: String) -> IoK8sKubernetesPkgApiV1PersistentVolumeStatus {
    self.phase = Some(phase);
    self
  }

  pub fn phase(&self) -> Option<&String> {
    self.phase.as_ref()
  }

  pub fn reset_phase(&mut self) {
    self.phase = None;
  }

  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> IoK8sKubernetesPkgApiV1PersistentVolumeStatus {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

}



