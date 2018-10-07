/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApisAuthorizationV1SelfSubjectAccessReviewSpec : SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisAuthorizationV1SelfSubjectAccessReviewSpec {
  #[serde(rename = "nonResourceAttributes")]
  non_resource_attributes: Option<::models::IoK8sKubernetesPkgApisAuthorizationV1NonResourceAttributes>,
  #[serde(rename = "resourceAttributes")]
  resource_attributes: Option<::models::IoK8sKubernetesPkgApisAuthorizationV1ResourceAttributes>
}

impl IoK8sKubernetesPkgApisAuthorizationV1SelfSubjectAccessReviewSpec {
  /// SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
  pub fn new() -> IoK8sKubernetesPkgApisAuthorizationV1SelfSubjectAccessReviewSpec {
    IoK8sKubernetesPkgApisAuthorizationV1SelfSubjectAccessReviewSpec {
      non_resource_attributes: None,
      resource_attributes: None
    }
  }

  pub fn set_non_resource_attributes(&mut self, non_resource_attributes: ::models::IoK8sKubernetesPkgApisAuthorizationV1NonResourceAttributes) {
    self.non_resource_attributes = Some(non_resource_attributes);
  }

  pub fn with_non_resource_attributes(mut self, non_resource_attributes: ::models::IoK8sKubernetesPkgApisAuthorizationV1NonResourceAttributes) -> IoK8sKubernetesPkgApisAuthorizationV1SelfSubjectAccessReviewSpec {
    self.non_resource_attributes = Some(non_resource_attributes);
    self
  }

  pub fn non_resource_attributes(&self) -> Option<&::models::IoK8sKubernetesPkgApisAuthorizationV1NonResourceAttributes> {
    self.non_resource_attributes.as_ref()
  }

  pub fn reset_non_resource_attributes(&mut self) {
    self.non_resource_attributes = None;
  }

  pub fn set_resource_attributes(&mut self, resource_attributes: ::models::IoK8sKubernetesPkgApisAuthorizationV1ResourceAttributes) {
    self.resource_attributes = Some(resource_attributes);
  }

  pub fn with_resource_attributes(mut self, resource_attributes: ::models::IoK8sKubernetesPkgApisAuthorizationV1ResourceAttributes) -> IoK8sKubernetesPkgApisAuthorizationV1SelfSubjectAccessReviewSpec {
    self.resource_attributes = Some(resource_attributes);
    self
  }

  pub fn resource_attributes(&self) -> Option<&::models::IoK8sKubernetesPkgApisAuthorizationV1ResourceAttributes> {
    self.resource_attributes.as_ref()
  }

  pub fn reset_resource_attributes(&mut self) {
    self.resource_attributes = None;
  }

}



