/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V2beta1ObjectMetricStatus : ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V2beta1ObjectMetricStatus {
  /// averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
  #[serde(rename = "averageValue")]
  average_value: Option<String>,
  /// currentValue is the current value of the metric (as a quantity).
  #[serde(rename = "currentValue")]
  current_value: String,
  /// metricName is the name of the metric in question.
  #[serde(rename = "metricName")]
  metric_name: String,
  /// selector is the string-encoded form of a standard kubernetes label selector for the given metric When set in the ObjectMetricSource, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
  #[serde(rename = "selector")]
  selector: Option<::models::V1LabelSelector>,
  /// target is the described Kubernetes object.
  #[serde(rename = "target")]
  target: ::models::V2beta1CrossVersionObjectReference
}

impl V2beta1ObjectMetricStatus {
  /// ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
  pub fn new(current_value: String, metric_name: String, target: ::models::V2beta1CrossVersionObjectReference) -> V2beta1ObjectMetricStatus {
    V2beta1ObjectMetricStatus {
      average_value: None,
      current_value: current_value,
      metric_name: metric_name,
      selector: None,
      target: target
    }
  }

  pub fn set_average_value(&mut self, average_value: String) {
    self.average_value = Some(average_value);
  }

  pub fn with_average_value(mut self, average_value: String) -> V2beta1ObjectMetricStatus {
    self.average_value = Some(average_value);
    self
  }

  pub fn average_value(&self) -> Option<&String> {
    self.average_value.as_ref()
  }

  pub fn reset_average_value(&mut self) {
    self.average_value = None;
  }

  pub fn set_current_value(&mut self, current_value: String) {
    self.current_value = current_value;
  }

  pub fn with_current_value(mut self, current_value: String) -> V2beta1ObjectMetricStatus {
    self.current_value = current_value;
    self
  }

  pub fn current_value(&self) -> &String {
    &self.current_value
  }


  pub fn set_metric_name(&mut self, metric_name: String) {
    self.metric_name = metric_name;
  }

  pub fn with_metric_name(mut self, metric_name: String) -> V2beta1ObjectMetricStatus {
    self.metric_name = metric_name;
    self
  }

  pub fn metric_name(&self) -> &String {
    &self.metric_name
  }


  pub fn set_selector(&mut self, selector: ::models::V1LabelSelector) {
    self.selector = Some(selector);
  }

  pub fn with_selector(mut self, selector: ::models::V1LabelSelector) -> V2beta1ObjectMetricStatus {
    self.selector = Some(selector);
    self
  }

  pub fn selector(&self) -> Option<&::models::V1LabelSelector> {
    self.selector.as_ref()
  }

  pub fn reset_selector(&mut self) {
    self.selector = None;
  }

  pub fn set_target(&mut self, target: ::models::V2beta1CrossVersionObjectReference) {
    self.target = target;
  }

  pub fn with_target(mut self, target: ::models::V2beta1CrossVersionObjectReference) -> V2beta1ObjectMetricStatus {
    self.target = target;
    self
  }

  pub fn target(&self) -> &::models::V2beta1CrossVersionObjectReference {
    &self.target
  }


}


