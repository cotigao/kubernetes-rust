/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding : ClusterRoleBinding references a ClusterRole, but not contain it.  It can reference a ClusterRole in the global namespace, and adds who information via Subject.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  #[serde(rename = "metadata")]
  metadata: Option<::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
  #[serde(rename = "roleRef")]
  role_ref: ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleRef,
  /// Subjects holds references to the objects the role applies to.
  #[serde(rename = "subjects")]
  subjects: Vec<::models::IoK8sKubernetesPkgApisRbacV1beta1Subject>
}

impl IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding {
  /// ClusterRoleBinding references a ClusterRole, but not contain it.  It can reference a ClusterRole in the global namespace, and adds who information via Subject.
  pub fn new(role_ref: ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleRef, subjects: Vec<::models::IoK8sKubernetesPkgApisRbacV1beta1Subject>) -> IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding {
    IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding {
      api_version: None,
      kind: None,
      metadata: None,
      role_ref: role_ref,
      subjects: subjects
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding {
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

  pub fn with_kind(mut self, kind: String) -> IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding {
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

  pub fn with_metadata(mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) -> IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_role_ref(&mut self, role_ref: ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleRef) {
    self.role_ref = role_ref;
  }

  pub fn with_role_ref(mut self, role_ref: ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleRef) -> IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding {
    self.role_ref = role_ref;
    self
  }

  pub fn role_ref(&self) -> &::models::IoK8sKubernetesPkgApisRbacV1beta1RoleRef {
    &self.role_ref
  }


  pub fn set_subjects(&mut self, subjects: Vec<::models::IoK8sKubernetesPkgApisRbacV1beta1Subject>) {
    self.subjects = subjects;
  }

  pub fn with_subjects(mut self, subjects: Vec<::models::IoK8sKubernetesPkgApisRbacV1beta1Subject>) -> IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding {
    self.subjects = subjects;
    self
  }

  pub fn subjects(&self) -> &Vec<::models::IoK8sKubernetesPkgApisRbacV1beta1Subject> {
    &self.subjects
  }


}



