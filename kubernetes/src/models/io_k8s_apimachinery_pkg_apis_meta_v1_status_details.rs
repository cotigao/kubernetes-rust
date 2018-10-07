/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApimachineryPkgApisMetaV1StatusDetails : StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApimachineryPkgApisMetaV1StatusDetails {
  /// The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.
  #[serde(rename = "causes")]
  causes: Option<Vec<::models::IoK8sApimachineryPkgApisMetaV1StatusCause>>,
  /// The group attribute of the resource associated with the status StatusReason.
  #[serde(rename = "group")]
  group: Option<String>,
  /// The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).
  #[serde(rename = "name")]
  name: Option<String>,
  /// If specified, the time in seconds before the operation should be retried.
  #[serde(rename = "retryAfterSeconds")]
  retry_after_seconds: Option<i32>,
  /// UID of the resource. (when there is a single resource which can be described). More info: http://kubernetes.io/docs/user-guide/identifiers#uids
  #[serde(rename = "uid")]
  uid: Option<String>
}

impl IoK8sApimachineryPkgApisMetaV1StatusDetails {
  /// StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.
  pub fn new() -> IoK8sApimachineryPkgApisMetaV1StatusDetails {
    IoK8sApimachineryPkgApisMetaV1StatusDetails {
      causes: None,
      group: None,
      kind: None,
      name: None,
      retry_after_seconds: None,
      uid: None
    }
  }

  pub fn set_causes(&mut self, causes: Vec<::models::IoK8sApimachineryPkgApisMetaV1StatusCause>) {
    self.causes = Some(causes);
  }

  pub fn with_causes(mut self, causes: Vec<::models::IoK8sApimachineryPkgApisMetaV1StatusCause>) -> IoK8sApimachineryPkgApisMetaV1StatusDetails {
    self.causes = Some(causes);
    self
  }

  pub fn causes(&self) -> Option<&Vec<::models::IoK8sApimachineryPkgApisMetaV1StatusCause>> {
    self.causes.as_ref()
  }

  pub fn reset_causes(&mut self) {
    self.causes = None;
  }

  pub fn set_group(&mut self, group: String) {
    self.group = Some(group);
  }

  pub fn with_group(mut self, group: String) -> IoK8sApimachineryPkgApisMetaV1StatusDetails {
    self.group = Some(group);
    self
  }

  pub fn group(&self) -> Option<&String> {
    self.group.as_ref()
  }

  pub fn reset_group(&mut self) {
    self.group = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> IoK8sApimachineryPkgApisMetaV1StatusDetails {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> IoK8sApimachineryPkgApisMetaV1StatusDetails {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_retry_after_seconds(&mut self, retry_after_seconds: i32) {
    self.retry_after_seconds = Some(retry_after_seconds);
  }

  pub fn with_retry_after_seconds(mut self, retry_after_seconds: i32) -> IoK8sApimachineryPkgApisMetaV1StatusDetails {
    self.retry_after_seconds = Some(retry_after_seconds);
    self
  }

  pub fn retry_after_seconds(&self) -> Option<&i32> {
    self.retry_after_seconds.as_ref()
  }

  pub fn reset_retry_after_seconds(&mut self) {
    self.retry_after_seconds = None;
  }

  pub fn set_uid(&mut self, uid: String) {
    self.uid = Some(uid);
  }

  pub fn with_uid(mut self, uid: String) -> IoK8sApimachineryPkgApisMetaV1StatusDetails {
    self.uid = Some(uid);
    self
  }

  pub fn uid(&self) -> Option<&String> {
    self.uid.as_ref()
  }

  pub fn reset_uid(&mut self) {
    self.uid = None;
  }

}



