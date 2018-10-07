/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApiV1VolumeProjection : Projection that may be projected along with other supported volume types

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1VolumeProjection {
  #[serde(rename = "configMap")]
  config_map: Option<::models::IoK8sKubernetesPkgApiV1ConfigMapProjection>,
  #[serde(rename = "downwardAPI")]
  downward_api: Option<::models::IoK8sKubernetesPkgApiV1DownwardApiProjection>,
  #[serde(rename = "secret")]
  secret: Option<::models::IoK8sKubernetesPkgApiV1SecretProjection>
}

impl IoK8sKubernetesPkgApiV1VolumeProjection {
  /// Projection that may be projected along with other supported volume types
  pub fn new() -> IoK8sKubernetesPkgApiV1VolumeProjection {
    IoK8sKubernetesPkgApiV1VolumeProjection {
      config_map: None,
      downward_api: None,
      secret: None
    }
  }

  pub fn set_config_map(&mut self, config_map: ::models::IoK8sKubernetesPkgApiV1ConfigMapProjection) {
    self.config_map = Some(config_map);
  }

  pub fn with_config_map(mut self, config_map: ::models::IoK8sKubernetesPkgApiV1ConfigMapProjection) -> IoK8sKubernetesPkgApiV1VolumeProjection {
    self.config_map = Some(config_map);
    self
  }

  pub fn config_map(&self) -> Option<&::models::IoK8sKubernetesPkgApiV1ConfigMapProjection> {
    self.config_map.as_ref()
  }

  pub fn reset_config_map(&mut self) {
    self.config_map = None;
  }

  pub fn set_downward_api(&mut self, downward_api: ::models::IoK8sKubernetesPkgApiV1DownwardApiProjection) {
    self.downward_api = Some(downward_api);
  }

  pub fn with_downward_api(mut self, downward_api: ::models::IoK8sKubernetesPkgApiV1DownwardApiProjection) -> IoK8sKubernetesPkgApiV1VolumeProjection {
    self.downward_api = Some(downward_api);
    self
  }

  pub fn downward_api(&self) -> Option<&::models::IoK8sKubernetesPkgApiV1DownwardApiProjection> {
    self.downward_api.as_ref()
  }

  pub fn reset_downward_api(&mut self) {
    self.downward_api = None;
  }

  pub fn set_secret(&mut self, secret: ::models::IoK8sKubernetesPkgApiV1SecretProjection) {
    self.secret = Some(secret);
  }

  pub fn with_secret(mut self, secret: ::models::IoK8sKubernetesPkgApiV1SecretProjection) -> IoK8sKubernetesPkgApiV1VolumeProjection {
    self.secret = Some(secret);
    self
  }

  pub fn secret(&self) -> Option<&::models::IoK8sKubernetesPkgApiV1SecretProjection> {
    self.secret.as_ref()
  }

  pub fn reset_secret(&mut self) {
    self.secret = None;
  }

}



