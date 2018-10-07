/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicyList : Pod Security Policy List is a list of PodSecurityPolicy objects.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicyList {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Items is a list of schema objects.
  #[serde(rename = "items")]
  items: Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicy>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  #[serde(rename = "metadata")]
  metadata: Option<::models::IoK8sApimachineryPkgApisMetaV1ListMeta>
}

impl IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicyList {
  /// Pod Security Policy List is a list of PodSecurityPolicy objects.
  pub fn new(items: Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicy>) -> IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicyList {
    IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicyList {
      api_version: None,
      items: items,
      kind: None,
      metadata: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicyList {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_items(&mut self, items: Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicy>) {
    self.items = items;
  }

  pub fn with_items(mut self, items: Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicy>) -> IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicyList {
    self.items = items;
    self
  }

  pub fn items(&self) -> &Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicy> {
    &self.items
  }


  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicyList {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ListMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ListMeta) -> IoK8sKubernetesPkgApisExtensionsV1beta1PodSecurityPolicyList {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1ListMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

}



