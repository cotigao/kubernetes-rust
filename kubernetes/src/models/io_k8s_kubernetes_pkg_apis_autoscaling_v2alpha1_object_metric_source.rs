/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource : ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource {
  /// metricName is the name of the metric in question.
  #[serde(rename = "metricName")]
  metric_name: String,
  #[serde(rename = "target")]
  target: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1CrossVersionObjectReference,
  #[serde(rename = "targetValue")]
  target_value: String
}

impl IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource {
  /// ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
  pub fn new(metric_name: String, target: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1CrossVersionObjectReference, target_value: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource {
    IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource {
      metric_name: metric_name,
      target: target,
      target_value: target_value
    }
  }

  pub fn set_metric_name(&mut self, metric_name: String) {
    self.metric_name = metric_name;
  }

  pub fn with_metric_name(mut self, metric_name: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource {
    self.metric_name = metric_name;
    self
  }

  pub fn metric_name(&self) -> &String {
    &self.metric_name
  }


  pub fn set_target(&mut self, target: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1CrossVersionObjectReference) {
    self.target = target;
  }

  pub fn with_target(mut self, target: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1CrossVersionObjectReference) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource {
    self.target = target;
    self
  }

  pub fn target(&self) -> &::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1CrossVersionObjectReference {
    &self.target
  }


  pub fn set_target_value(&mut self, target_value: String) {
    self.target_value = target_value;
  }

  pub fn with_target_value(mut self, target_value: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource {
    self.target_value = target_value;
    self
  }

  pub fn target_value(&self) -> &String {
    &self.target_value
  }


}



