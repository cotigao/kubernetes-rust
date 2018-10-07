/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApiV1ServiceAccount : ServiceAccount binds together: * a name, understood by users, and perhaps by peripheral systems, for an identity * a principal that can be authenticated and authorized * a set of secrets

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1ServiceAccount {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// AutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted. Can be overridden at the pod level.
  #[serde(rename = "automountServiceAccountToken")]
  automount_service_account_token: Option<bool>,
  /// ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images in pods that reference this ServiceAccount. ImagePullSecrets are distinct from Secrets because Secrets can be mounted in the pod, but ImagePullSecrets are only accessed by the kubelet. More info: https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod
  #[serde(rename = "imagePullSecrets")]
  image_pull_secrets: Option<Vec<::models::IoK8sKubernetesPkgApiV1LocalObjectReference>>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  #[serde(rename = "metadata")]
  metadata: Option<::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
  /// Secrets is the list of secrets allowed to be used by pods running using this ServiceAccount. More info: https://kubernetes.io/docs/concepts/configuration/secret
  #[serde(rename = "secrets")]
  secrets: Option<Vec<::models::IoK8sKubernetesPkgApiV1ObjectReference>>
}

impl IoK8sKubernetesPkgApiV1ServiceAccount {
  /// ServiceAccount binds together: * a name, understood by users, and perhaps by peripheral systems, for an identity * a principal that can be authenticated and authorized * a set of secrets
  pub fn new() -> IoK8sKubernetesPkgApiV1ServiceAccount {
    IoK8sKubernetesPkgApiV1ServiceAccount {
      api_version: None,
      automount_service_account_token: None,
      image_pull_secrets: None,
      kind: None,
      metadata: None,
      secrets: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> IoK8sKubernetesPkgApiV1ServiceAccount {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_automount_service_account_token(&mut self, automount_service_account_token: bool) {
    self.automount_service_account_token = Some(automount_service_account_token);
  }

  pub fn with_automount_service_account_token(mut self, automount_service_account_token: bool) -> IoK8sKubernetesPkgApiV1ServiceAccount {
    self.automount_service_account_token = Some(automount_service_account_token);
    self
  }

  pub fn automount_service_account_token(&self) -> Option<&bool> {
    self.automount_service_account_token.as_ref()
  }

  pub fn reset_automount_service_account_token(&mut self) {
    self.automount_service_account_token = None;
  }

  pub fn set_image_pull_secrets(&mut self, image_pull_secrets: Vec<::models::IoK8sKubernetesPkgApiV1LocalObjectReference>) {
    self.image_pull_secrets = Some(image_pull_secrets);
  }

  pub fn with_image_pull_secrets(mut self, image_pull_secrets: Vec<::models::IoK8sKubernetesPkgApiV1LocalObjectReference>) -> IoK8sKubernetesPkgApiV1ServiceAccount {
    self.image_pull_secrets = Some(image_pull_secrets);
    self
  }

  pub fn image_pull_secrets(&self) -> Option<&Vec<::models::IoK8sKubernetesPkgApiV1LocalObjectReference>> {
    self.image_pull_secrets.as_ref()
  }

  pub fn reset_image_pull_secrets(&mut self) {
    self.image_pull_secrets = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> IoK8sKubernetesPkgApiV1ServiceAccount {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) -> IoK8sKubernetesPkgApiV1ServiceAccount {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_secrets(&mut self, secrets: Vec<::models::IoK8sKubernetesPkgApiV1ObjectReference>) {
    self.secrets = Some(secrets);
  }

  pub fn with_secrets(mut self, secrets: Vec<::models::IoK8sKubernetesPkgApiV1ObjectReference>) -> IoK8sKubernetesPkgApiV1ServiceAccount {
    self.secrets = Some(secrets);
    self
  }

  pub fn secrets(&self) -> Option<&Vec<::models::IoK8sKubernetesPkgApiV1ObjectReference>> {
    self.secrets.as_ref()
  }

  pub fn reset_secrets(&mut self) {
    self.secrets = None;
  }

}



