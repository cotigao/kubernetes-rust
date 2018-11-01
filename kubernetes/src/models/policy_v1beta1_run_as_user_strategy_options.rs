/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PolicyV1beta1RunAsUserStrategyOptions : RunAsUserStrategyOptions defines the strategy type and any options used to create the strategy.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyV1beta1RunAsUserStrategyOptions {
  /// ranges are the allowed ranges of uids that may be used. If you would like to force a single uid then supply a single range with the same start and end. Required for MustRunAs.
  #[serde(rename = "ranges")]
  ranges: Option<Vec<::models::PolicyV1beta1IdRange>>,
  /// rule is the strategy that will dictate the allowable RunAsUser values that may be set.
  #[serde(rename = "rule")]
  rule: String
}

impl PolicyV1beta1RunAsUserStrategyOptions {
  /// RunAsUserStrategyOptions defines the strategy type and any options used to create the strategy.
  pub fn new(rule: String) -> PolicyV1beta1RunAsUserStrategyOptions {
    PolicyV1beta1RunAsUserStrategyOptions {
      ranges: None,
      rule: rule
    }
  }

  pub fn set_ranges(&mut self, ranges: Vec<::models::PolicyV1beta1IdRange>) {
    self.ranges = Some(ranges);
  }

  pub fn with_ranges(mut self, ranges: Vec<::models::PolicyV1beta1IdRange>) -> PolicyV1beta1RunAsUserStrategyOptions {
    self.ranges = Some(ranges);
    self
  }

  pub fn ranges(&self) -> Option<&Vec<::models::PolicyV1beta1IdRange>> {
    self.ranges.as_ref()
  }

  pub fn reset_ranges(&mut self) {
    self.ranges = None;
  }

  pub fn set_rule(&mut self, rule: String) {
    self.rule = rule;
  }

  pub fn with_rule(mut self, rule: String) -> PolicyV1beta1RunAsUserStrategyOptions {
    self.rule = rule;
    self
  }

  pub fn rule(&self) -> &String {
    &self.rule
  }


}


