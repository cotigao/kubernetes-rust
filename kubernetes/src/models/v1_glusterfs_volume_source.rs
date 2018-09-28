/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1GlusterfsVolumeSource : Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1GlusterfsVolumeSource {
  /// EndpointsName is the endpoint name that details Glusterfs topology. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md#create-a-pod
  #[serde(rename = "endpoints")]
  endpoints: String,
  /// Path is the Glusterfs volume path. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md#create-a-pod
  #[serde(rename = "path")]
  path: String,
  /// ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md#create-a-pod
  #[serde(rename = "readOnly")]
  read_only: Option<bool>
}

impl V1GlusterfsVolumeSource {
  /// Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling.
  pub fn new(endpoints: String, path: String) -> V1GlusterfsVolumeSource {
    V1GlusterfsVolumeSource {
      endpoints: endpoints,
      path: path,
      read_only: None
    }
  }

  pub fn set_endpoints(&mut self, endpoints: String) {
    self.endpoints = endpoints;
  }

  pub fn with_endpoints(mut self, endpoints: String) -> V1GlusterfsVolumeSource {
    self.endpoints = endpoints;
    self
  }

  pub fn endpoints(&self) -> &String {
    &self.endpoints
  }


  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }

  pub fn with_path(mut self, path: String) -> V1GlusterfsVolumeSource {
    self.path = path;
    self
  }

  pub fn path(&self) -> &String {
    &self.path
  }


  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> V1GlusterfsVolumeSource {
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


