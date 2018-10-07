/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyIngressRule : NetworkPolicyIngressRule describes a particular set of traffic that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and from.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyIngressRule {
  /// List of sources which should be able to access the pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all sources (traffic not restricted by source). If this field is present and contains at least on item, this rule allows traffic only if the traffic matches at least one item in the from list.
  #[serde(rename = "from")]
  from: Option<Vec<::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyPeer>>,
  /// List of ports which should be made accessible on the pods selected for this rule. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
  #[serde(rename = "ports")]
  ports: Option<Vec<::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyPort>>
}

impl IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyIngressRule {
  /// NetworkPolicyIngressRule describes a particular set of traffic that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and from.
  pub fn new() -> IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyIngressRule {
    IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyIngressRule {
      from: None,
      ports: None
    }
  }

  pub fn set_from(&mut self, from: Vec<::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyPeer>) {
    self.from = Some(from);
  }

  pub fn with_from(mut self, from: Vec<::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyPeer>) -> IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyIngressRule {
    self.from = Some(from);
    self
  }

  pub fn from(&self) -> Option<&Vec<::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyPeer>> {
    self.from.as_ref()
  }

  pub fn reset_from(&mut self) {
    self.from = None;
  }

  pub fn set_ports(&mut self, ports: Vec<::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyPort>) {
    self.ports = Some(ports);
  }

  pub fn with_ports(mut self, ports: Vec<::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyPort>) -> IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyIngressRule {
    self.ports = Some(ports);
    self
  }

  pub fn ports(&self) -> Option<&Vec<::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyPort>> {
    self.ports.as_ref()
  }

  pub fn reset_ports(&mut self) {
    self.ports = None;
  }

}



