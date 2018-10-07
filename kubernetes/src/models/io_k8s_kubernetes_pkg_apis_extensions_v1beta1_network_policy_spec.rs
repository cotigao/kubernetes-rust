/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisExtensionsV1beta1NetworkPolicySpec {
  /// List of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod OR if the traffic source is the pod's local node, OR if the traffic matches at least one ingress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy does not allow any traffic (and serves solely to ensure that the pods it selects are isolated by default).
  #[serde(rename = "ingress")]
  ingress: Option<Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1NetworkPolicyIngressRule>>,
  #[serde(rename = "podSelector")]
  pod_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector
}

impl IoK8sKubernetesPkgApisExtensionsV1beta1NetworkPolicySpec {
  pub fn new(pod_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) -> IoK8sKubernetesPkgApisExtensionsV1beta1NetworkPolicySpec {
    IoK8sKubernetesPkgApisExtensionsV1beta1NetworkPolicySpec {
      ingress: None,
      pod_selector: pod_selector
    }
  }

  pub fn set_ingress(&mut self, ingress: Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1NetworkPolicyIngressRule>) {
    self.ingress = Some(ingress);
  }

  pub fn with_ingress(mut self, ingress: Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1NetworkPolicyIngressRule>) -> IoK8sKubernetesPkgApisExtensionsV1beta1NetworkPolicySpec {
    self.ingress = Some(ingress);
    self
  }

  pub fn ingress(&self) -> Option<&Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1NetworkPolicyIngressRule>> {
    self.ingress.as_ref()
  }

  pub fn reset_ingress(&mut self) {
    self.ingress = None;
  }

  pub fn set_pod_selector(&mut self, pod_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) {
    self.pod_selector = pod_selector;
  }

  pub fn with_pod_selector(mut self, pod_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) -> IoK8sKubernetesPkgApisExtensionsV1beta1NetworkPolicySpec {
    self.pod_selector = pod_selector;
    self
  }

  pub fn pod_selector(&self) -> &::models::IoK8sApimachineryPkgApisMetaV1LabelSelector {
    &self.pod_selector
  }


}



