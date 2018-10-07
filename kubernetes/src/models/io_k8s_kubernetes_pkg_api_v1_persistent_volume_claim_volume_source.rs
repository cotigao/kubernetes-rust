/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApiV1PersistentVolumeClaimVolumeSource : PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace. This volume finds the bound PV and mounts that volume for the pod. A PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of volume that is owned by someone else (the system).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1PersistentVolumeClaimVolumeSource {
  /// ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
  #[serde(rename = "claimName")]
  claim_name: String,
  /// Will force the ReadOnly setting in VolumeMounts. Default false.
  #[serde(rename = "readOnly")]
  read_only: Option<bool>
}

impl IoK8sKubernetesPkgApiV1PersistentVolumeClaimVolumeSource {
  /// PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace. This volume finds the bound PV and mounts that volume for the pod. A PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of volume that is owned by someone else (the system).
  pub fn new(claim_name: String) -> IoK8sKubernetesPkgApiV1PersistentVolumeClaimVolumeSource {
    IoK8sKubernetesPkgApiV1PersistentVolumeClaimVolumeSource {
      claim_name: claim_name,
      read_only: None
    }
  }

  pub fn set_claim_name(&mut self, claim_name: String) {
    self.claim_name = claim_name;
  }

  pub fn with_claim_name(mut self, claim_name: String) -> IoK8sKubernetesPkgApiV1PersistentVolumeClaimVolumeSource {
    self.claim_name = claim_name;
    self
  }

  pub fn claim_name(&self) -> &String {
    &self.claim_name
  }


  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> IoK8sKubernetesPkgApiV1PersistentVolumeClaimVolumeSource {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

}



