/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1VolumeAttachment : VolumeAttachment captures the intent to attach or detach the specified volume to/from the specified node.  VolumeAttachment objects are non-namespaced.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1VolumeAttachment {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Standard object metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
  #[serde(rename = "metadata")]
  metadata: Option<::models::V1ObjectMeta>,
  /// Specification of the desired attach/detach volume behavior. Populated by the Kubernetes system.
  #[serde(rename = "spec")]
  spec: ::models::V1beta1VolumeAttachmentSpec,
  /// Status of the VolumeAttachment request. Populated by the entity completing the attach or detach operation, i.e. the external-attacher.
  #[serde(rename = "status")]
  status: Option<::models::V1beta1VolumeAttachmentStatus>
}

impl V1beta1VolumeAttachment {
  /// VolumeAttachment captures the intent to attach or detach the specified volume to/from the specified node.  VolumeAttachment objects are non-namespaced.
  pub fn new(spec: ::models::V1beta1VolumeAttachmentSpec) -> V1beta1VolumeAttachment {
    V1beta1VolumeAttachment {
      api_version: None,
      kind: None,
      metadata: None,
      spec: spec,
      status: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> V1beta1VolumeAttachment {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> V1beta1VolumeAttachment {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::V1ObjectMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::V1ObjectMeta) -> V1beta1VolumeAttachment {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::V1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_spec(&mut self, spec: ::models::V1beta1VolumeAttachmentSpec) {
    self.spec = spec;
  }

  pub fn with_spec(mut self, spec: ::models::V1beta1VolumeAttachmentSpec) -> V1beta1VolumeAttachment {
    self.spec = spec;
    self
  }

  pub fn spec(&self) -> &::models::V1beta1VolumeAttachmentSpec {
    &self.spec
  }


  pub fn set_status(&mut self, status: ::models::V1beta1VolumeAttachmentStatus) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: ::models::V1beta1VolumeAttachmentStatus) -> V1beta1VolumeAttachment {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&::models::V1beta1VolumeAttachmentStatus> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}


