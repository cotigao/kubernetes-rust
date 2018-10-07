/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubernetesPkgApisExtensionsV1beta1FsGroupStrategyOptions : FSGroupStrategyOptions defines the strategy type and options used to create the strategy.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisExtensionsV1beta1FsGroupStrategyOptions {
  /// Ranges are the allowed ranges of fs groups.  If you would like to force a single fs group then supply a single range with the same start and end.
  #[serde(rename = "ranges")]
  ranges: Option<Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1IdRange>>,
  /// Rule is the strategy that will dictate what FSGroup is used in the SecurityContext.
  #[serde(rename = "rule")]
  rule: Option<String>
}

impl IoK8sKubernetesPkgApisExtensionsV1beta1FsGroupStrategyOptions {
  /// FSGroupStrategyOptions defines the strategy type and options used to create the strategy.
  pub fn new() -> IoK8sKubernetesPkgApisExtensionsV1beta1FsGroupStrategyOptions {
    IoK8sKubernetesPkgApisExtensionsV1beta1FsGroupStrategyOptions {
      ranges: None,
      rule: None
    }
  }

  pub fn set_ranges(&mut self, ranges: Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1IdRange>) {
    self.ranges = Some(ranges);
  }

  pub fn with_ranges(mut self, ranges: Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1IdRange>) -> IoK8sKubernetesPkgApisExtensionsV1beta1FsGroupStrategyOptions {
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
    self.rule = Some(rule);
  }

  pub fn with_rule(mut self, rule: String) -> IoK8sKubernetesPkgApisExtensionsV1beta1FsGroupStrategyOptions {
    self.rule = Some(rule);
    self
  }

  pub fn rule(&self) -> Option<&String> {
    self.rule.as_ref()
  }

  pub fn reset_rule(&mut self) {
    self.rule = None;
  }

}



