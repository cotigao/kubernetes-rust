/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApisAuthorizationV1SubjectAccessReviewSpec : SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisAuthorizationV1SubjectAccessReviewSpec {
  /// Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.
  #[serde(rename = "extra")]
  extra: Option<::std::collections::HashMap<String, Vec<String>>>,
  /// Groups is the groups you're testing for.
  #[serde(rename = "groups")]
  groups: Option<Vec<String>>,
  #[serde(rename = "nonResourceAttributes")]
  non_resource_attributes: Option<::models::IoK8sKubernetesPkgApisAuthorizationV1NonResourceAttributes>,
  #[serde(rename = "resourceAttributes")]
  resource_attributes: Option<::models::IoK8sKubernetesPkgApisAuthorizationV1ResourceAttributes>,
  /// User is the user you're testing for. If you specify \"User\" but not \"Groups\", then is it interpreted as \"What if User were not a member of any groups
  #[serde(rename = "user")]
  user: Option<String>
}

impl IoK8sKubernetesPkgApisAuthorizationV1SubjectAccessReviewSpec {
  /// SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
  pub fn new() -> IoK8sKubernetesPkgApisAuthorizationV1SubjectAccessReviewSpec {
    IoK8sKubernetesPkgApisAuthorizationV1SubjectAccessReviewSpec {
      extra: None,
      groups: None,
      non_resource_attributes: None,
      resource_attributes: None,
      user: None
    }
  }

  pub fn set_extra(&mut self, extra: ::std::collections::HashMap<String, Vec<String>>) {
    self.extra = Some(extra);
  }

  pub fn with_extra(mut self, extra: ::std::collections::HashMap<String, Vec<String>>) -> IoK8sKubernetesPkgApisAuthorizationV1SubjectAccessReviewSpec {
    self.extra = Some(extra);
    self
  }

  pub fn extra(&self) -> Option<&::std::collections::HashMap<String, Vec<String>>> {
    self.extra.as_ref()
  }

  pub fn reset_extra(&mut self) {
    self.extra = None;
  }

  pub fn set_groups(&mut self, groups: Vec<String>) {
    self.groups = Some(groups);
  }

  pub fn with_groups(mut self, groups: Vec<String>) -> IoK8sKubernetesPkgApisAuthorizationV1SubjectAccessReviewSpec {
    self.groups = Some(groups);
    self
  }

  pub fn groups(&self) -> Option<&Vec<String>> {
    self.groups.as_ref()
  }

  pub fn reset_groups(&mut self) {
    self.groups = None;
  }

  pub fn set_non_resource_attributes(&mut self, non_resource_attributes: ::models::IoK8sKubernetesPkgApisAuthorizationV1NonResourceAttributes) {
    self.non_resource_attributes = Some(non_resource_attributes);
  }

  pub fn with_non_resource_attributes(mut self, non_resource_attributes: ::models::IoK8sKubernetesPkgApisAuthorizationV1NonResourceAttributes) -> IoK8sKubernetesPkgApisAuthorizationV1SubjectAccessReviewSpec {
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

  pub fn with_resource_attributes(mut self, resource_attributes: ::models::IoK8sKubernetesPkgApisAuthorizationV1ResourceAttributes) -> IoK8sKubernetesPkgApisAuthorizationV1SubjectAccessReviewSpec {
    self.resource_attributes = Some(resource_attributes);
    self
  }

  pub fn resource_attributes(&self) -> Option<&::models::IoK8sKubernetesPkgApisAuthorizationV1ResourceAttributes> {
    self.resource_attributes.as_ref()
  }

  pub fn reset_resource_attributes(&mut self) {
    self.resource_attributes = None;
  }

  pub fn set_user(&mut self, user: String) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: String) -> IoK8sKubernetesPkgApisAuthorizationV1SubjectAccessReviewSpec {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&String> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}



