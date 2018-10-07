/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApisExtensionsV1beta1RunAsUserStrategyOptions : Run A sUser Strategy Options defines the strategy type and any options used to create the strategy.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisExtensionsV1beta1RunAsUserStrategyOptions {
  /// Ranges are the allowed ranges of uids that may be used.
  #[serde(rename = "ranges")]
  ranges: Option<Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1IdRange>>,
  /// Rule is the strategy that will dictate the allowable RunAsUser values that may be set.
  #[serde(rename = "rule")]
  rule: String
}

impl IoK8sKubernetesPkgApisExtensionsV1beta1RunAsUserStrategyOptions {
  /// Run A sUser Strategy Options defines the strategy type and any options used to create the strategy.
  pub fn new(rule: String) -> IoK8sKubernetesPkgApisExtensionsV1beta1RunAsUserStrategyOptions {
    IoK8sKubernetesPkgApisExtensionsV1beta1RunAsUserStrategyOptions {
      ranges: None,
      rule: rule
    }
  }

  pub fn set_ranges(&mut self, ranges: Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1IdRange>) {
    self.ranges = Some(ranges);
  }

  pub fn with_ranges(mut self, ranges: Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1IdRange>) -> IoK8sKubernetesPkgApisExtensionsV1beta1RunAsUserStrategyOptions {
    self.ranges = Some(ranges);
    self
  }

  pub fn ranges(&self) -> Option<&Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1IdRange>> {
    self.ranges.as_ref()
  }

  pub fn reset_ranges(&mut self) {
    self.ranges = None;
  }

  pub fn set_rule(&mut self, rule: String) {
    self.rule = rule;
  }

  pub fn with_rule(mut self, rule: String) -> IoK8sKubernetesPkgApisExtensionsV1beta1RunAsUserStrategyOptions {
    self.rule = rule;
    self
  }

  pub fn rule(&self) -> &String {
    &self.rule
  }


}



