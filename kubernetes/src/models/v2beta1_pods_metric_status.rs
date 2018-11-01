/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V2beta1PodsMetricStatus : PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V2beta1PodsMetricStatus {
  /// currentAverageValue is the current value of the average of the metric across all relevant pods (as a quantity)
  #[serde(rename = "currentAverageValue")]
  current_average_value: String,
  /// metricName is the name of the metric in question
  #[serde(rename = "metricName")]
  metric_name: String,
  /// selector is the string-encoded form of a standard kubernetes label selector for the given metric When set in the PodsMetricSource, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
  #[serde(rename = "selector")]
  selector: Option<::models::V1LabelSelector>
}

impl V2beta1PodsMetricStatus {
  /// PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
  pub fn new(current_average_value: String, metric_name: String) -> V2beta1PodsMetricStatus {
    V2beta1PodsMetricStatus {
      current_average_value: current_average_value,
      metric_name: metric_name,
      selector: None
    }
  }

  pub fn set_current_average_value(&mut self, current_average_value: String) {
    self.current_average_value = current_average_value;
  }

  pub fn with_current_average_value(mut self, current_average_value: String) -> V2beta1PodsMetricStatus {
    self.current_average_value = current_average_value;
    self
  }

  pub fn current_average_value(&self) -> &String {
    &self.current_average_value
  }


  pub fn set_metric_name(&mut self, metric_name: String) {
    self.metric_name = metric_name;
  }

  pub fn with_metric_name(mut self, metric_name: String) -> V2beta1PodsMetricStatus {
    self.metric_name = metric_name;
    self
  }

  pub fn metric_name(&self) -> &String {
    &self.metric_name
  }


  pub fn set_selector(&mut self, selector: ::models::V1LabelSelector) {
    self.selector = Some(selector);
  }

  pub fn with_selector(mut self, selector: ::models::V1LabelSelector) -> V2beta1PodsMetricStatus {
    self.selector = Some(selector);
    self
  }

  pub fn selector(&self) -> Option<&::models::V1LabelSelector> {
    self.selector.as_ref()
  }

  pub fn reset_selector(&mut self) {
    self.selector = None;
  }

}


