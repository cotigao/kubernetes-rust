/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApiV1ServiceStatus : ServiceStatus represents the current status of a service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1ServiceStatus {
  /// LoadBalancer contains the current status of the load-balancer, if one is present.
  #[serde(rename = "loadBalancer")]
  load_balancer: Option<::models::IoK8sKubernetesPkgApiV1LoadBalancerStatus>
}

impl IoK8sKubernetesPkgApiV1ServiceStatus {
  /// ServiceStatus represents the current status of a service.
  pub fn new() -> IoK8sKubernetesPkgApiV1ServiceStatus {
    IoK8sKubernetesPkgApiV1ServiceStatus {
      load_balancer: None
    }
  }

  pub fn set_load_balancer(&mut self, load_balancer: ::models::IoK8sKubernetesPkgApiV1LoadBalancerStatus) {
    self.load_balancer = Some(load_balancer);
  }

  pub fn with_load_balancer(mut self, load_balancer: ::models::IoK8sKubernetesPkgApiV1LoadBalancerStatus) -> IoK8sKubernetesPkgApiV1ServiceStatus {
    self.load_balancer = Some(load_balancer);
    self
  }

  pub fn load_balancer(&self) -> Option<&::models::IoK8sKubernetesPkgApiV1LoadBalancerStatus> {
    self.load_balancer.as_ref()
  }

  pub fn reset_load_balancer(&mut self) {
    self.load_balancer = None;
  }

}


