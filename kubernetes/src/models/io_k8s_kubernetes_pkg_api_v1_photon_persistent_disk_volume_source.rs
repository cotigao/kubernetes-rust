/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApiV1PhotonPersistentDiskVolumeSource : Represents a Photon Controller persistent disk resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1PhotonPersistentDiskVolumeSource {
  /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// ID that identifies Photon Controller persistent disk
  #[serde(rename = "pdID")]
  pd_id: String
}

impl IoK8sKubernetesPkgApiV1PhotonPersistentDiskVolumeSource {
  /// Represents a Photon Controller persistent disk resource.
  pub fn new(pd_id: String) -> IoK8sKubernetesPkgApiV1PhotonPersistentDiskVolumeSource {
    IoK8sKubernetesPkgApiV1PhotonPersistentDiskVolumeSource {
      fs_type: None,
      pd_id: pd_id
    }
  }

  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> IoK8sKubernetesPkgApiV1PhotonPersistentDiskVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_pd_id(&mut self, pd_id: String) {
    self.pd_id = pd_id;
  }

  pub fn with_pd_id(mut self, pd_id: String) -> IoK8sKubernetesPkgApiV1PhotonPersistentDiskVolumeSource {
    self.pd_id = pd_id;
    self
  }

  pub fn pd_id(&self) -> &String {
    &self.pd_id
  }


}



