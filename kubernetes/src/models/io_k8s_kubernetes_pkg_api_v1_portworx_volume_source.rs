/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApiV1PortworxVolumeSource : PortworxVolumeSource represents a Portworx volume resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1PortworxVolumeSource {
  /// FSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\". Implicitly inferred to be \"ext4\" if unspecified.
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
  #[serde(rename = "readOnly")]
  read_only: Option<bool>,
  /// VolumeID uniquely identifies a Portworx volume
  #[serde(rename = "volumeID")]
  volume_id: String
}

impl IoK8sKubernetesPkgApiV1PortworxVolumeSource {
  /// PortworxVolumeSource represents a Portworx volume resource.
  pub fn new(volume_id: String) -> IoK8sKubernetesPkgApiV1PortworxVolumeSource {
    IoK8sKubernetesPkgApiV1PortworxVolumeSource {
      fs_type: None,
      read_only: None,
      volume_id: volume_id
    }
  }

  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> IoK8sKubernetesPkgApiV1PortworxVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> IoK8sKubernetesPkgApiV1PortworxVolumeSource {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

  pub fn set_volume_id(&mut self, volume_id: String) {
    self.volume_id = volume_id;
  }

  pub fn with_volume_id(mut self, volume_id: String) -> IoK8sKubernetesPkgApiV1PortworxVolumeSource {
    self.volume_id = volume_id;
    self
  }

  pub fn volume_id(&self) -> &String {
    &self.volume_id
  }


}



