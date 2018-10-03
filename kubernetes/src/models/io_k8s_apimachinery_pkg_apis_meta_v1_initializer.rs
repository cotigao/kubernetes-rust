/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sApimachineryPkgApisMetaV1Initializer : Initializer is information about an initializer that has not yet completed.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApimachineryPkgApisMetaV1Initializer {
  /// name of the process that is responsible for initializing this object.
  #[serde(rename = "name")]
  name: String
}

impl IoK8sApimachineryPkgApisMetaV1Initializer {
  /// Initializer is information about an initializer that has not yet completed.
  pub fn new(name: String) -> IoK8sApimachineryPkgApisMetaV1Initializer {
    IoK8sApimachineryPkgApisMetaV1Initializer {
      name: name
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> IoK8sApimachineryPkgApisMetaV1Initializer {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}


