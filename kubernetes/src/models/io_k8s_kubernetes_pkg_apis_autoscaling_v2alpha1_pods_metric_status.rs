/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricStatus : PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricStatus {
  #[serde(rename = "currentAverageValue")]
  current_average_value: String,
  /// metricName is the name of the metric in question
  #[serde(rename = "metricName")]
  metric_name: String
}

impl IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricStatus {
  /// PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
  pub fn new(current_average_value: String, metric_name: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricStatus {
    IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricStatus {
      current_average_value: current_average_value,
      metric_name: metric_name
    }
  }

  pub fn set_current_average_value(&mut self, current_average_value: String) {
    self.current_average_value = current_average_value;
  }

  pub fn with_current_average_value(mut self, current_average_value: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricStatus {
    self.current_average_value = current_average_value;
    self
  }

  pub fn current_average_value(&self) -> &String {
    &self.current_average_value
  }


  pub fn set_metric_name(&mut self, metric_name: String) {
    self.metric_name = metric_name;
  }

  pub fn with_metric_name(mut self, metric_name: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricStatus {
    self.metric_name = metric_name;
    self
  }

  pub fn metric_name(&self) -> &String {
    &self.metric_name
  }


}



