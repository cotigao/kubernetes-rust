/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1PodSecurityPolicySpec : Pod Security Policy Spec defines the policy enforced.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1PodSecurityPolicySpec {
  /// AllowedCapabilities is a list of capabilities that can be requested to add to the container. Capabilities in this field may be added at the pod author's discretion. You must not list a capability in both AllowedCapabilities and RequiredDropCapabilities.
  #[serde(rename = "allowedCapabilities")]
  allowed_capabilities: Option<Vec<String>>,
  /// DefaultAddCapabilities is the default set of capabilities that will be added to the container unless the pod spec specifically drops the capability.  You may not list a capabiility in both DefaultAddCapabilities and RequiredDropCapabilities.
  #[serde(rename = "defaultAddCapabilities")]
  default_add_capabilities: Option<Vec<String>>,
  /// FSGroup is the strategy that will dictate what fs group is used by the SecurityContext.
  #[serde(rename = "fsGroup")]
  fs_group: ::models::V1beta1FsGroupStrategyOptions,
  /// hostIPC determines if the policy allows the use of HostIPC in the pod spec.
  #[serde(rename = "hostIPC")]
  host_ipc: Option<bool>,
  /// hostNetwork determines if the policy allows the use of HostNetwork in the pod spec.
  #[serde(rename = "hostNetwork")]
  host_network: Option<bool>,
  /// hostPID determines if the policy allows the use of HostPID in the pod spec.
  #[serde(rename = "hostPID")]
  host_pid: Option<bool>,
  /// hostPorts determines which host port ranges are allowed to be exposed.
  #[serde(rename = "hostPorts")]
  host_ports: Option<Vec<::models::V1beta1HostPortRange>>,
  /// privileged determines if a pod can request to be run as privileged.
  #[serde(rename = "privileged")]
  privileged: Option<bool>,
  /// ReadOnlyRootFilesystem when set to true will force containers to run with a read only root file system.  If the container specifically requests to run with a non-read only root file system the PSP should deny the pod. If set to false the container may run with a read only root file system if it wishes but it will not be forced to.
  #[serde(rename = "readOnlyRootFilesystem")]
  read_only_root_filesystem: Option<bool>,
  /// RequiredDropCapabilities are the capabilities that will be dropped from the container.  These are required to be dropped and cannot be added.
  #[serde(rename = "requiredDropCapabilities")]
  required_drop_capabilities: Option<Vec<String>>,
  /// runAsUser is the strategy that will dictate the allowable RunAsUser values that may be set.
  #[serde(rename = "runAsUser")]
  run_as_user: ::models::V1beta1RunAsUserStrategyOptions,
  /// seLinux is the strategy that will dictate the allowable labels that may be set.
  #[serde(rename = "seLinux")]
  se_linux: ::models::V1beta1SeLinuxStrategyOptions,
  /// SupplementalGroups is the strategy that will dictate what supplemental groups are used by the SecurityContext.
  #[serde(rename = "supplementalGroups")]
  supplemental_groups: ::models::V1beta1SupplementalGroupsStrategyOptions,
  /// volumes is a white list of allowed volume plugins.  Empty indicates that all plugins may be used.
  #[serde(rename = "volumes")]
  volumes: Option<Vec<String>>
}

impl V1beta1PodSecurityPolicySpec {
  /// Pod Security Policy Spec defines the policy enforced.
  pub fn new(fs_group: ::models::V1beta1FsGroupStrategyOptions, run_as_user: ::models::V1beta1RunAsUserStrategyOptions, se_linux: ::models::V1beta1SeLinuxStrategyOptions, supplemental_groups: ::models::V1beta1SupplementalGroupsStrategyOptions) -> V1beta1PodSecurityPolicySpec {
    V1beta1PodSecurityPolicySpec {
      allowed_capabilities: None,
      default_add_capabilities: None,
      fs_group: fs_group,
      host_ipc: None,
      host_network: None,
      host_pid: None,
      host_ports: None,
      privileged: None,
      read_only_root_filesystem: None,
      required_drop_capabilities: None,
      run_as_user: run_as_user,
      se_linux: se_linux,
      supplemental_groups: supplemental_groups,
      volumes: None
    }
  }

  pub fn set_allowed_capabilities(&mut self, allowed_capabilities: Vec<String>) {
    self.allowed_capabilities = Some(allowed_capabilities);
  }

  pub fn with_allowed_capabilities(mut self, allowed_capabilities: Vec<String>) -> V1beta1PodSecurityPolicySpec {
    self.allowed_capabilities = Some(allowed_capabilities);
    self
  }

  pub fn allowed_capabilities(&self) -> Option<&Vec<String>> {
    self.allowed_capabilities.as_ref()
  }

  pub fn reset_allowed_capabilities(&mut self) {
    self.allowed_capabilities = None;
  }

  pub fn set_default_add_capabilities(&mut self, default_add_capabilities: Vec<String>) {
    self.default_add_capabilities = Some(default_add_capabilities);
  }

  pub fn with_default_add_capabilities(mut self, default_add_capabilities: Vec<String>) -> V1beta1PodSecurityPolicySpec {
    self.default_add_capabilities = Some(default_add_capabilities);
    self
  }

  pub fn default_add_capabilities(&self) -> Option<&Vec<String>> {
    self.default_add_capabilities.as_ref()
  }

  pub fn reset_default_add_capabilities(&mut self) {
    self.default_add_capabilities = None;
  }

  pub fn set_fs_group(&mut self, fs_group: ::models::V1beta1FsGroupStrategyOptions) {
    self.fs_group = fs_group;
  }

  pub fn with_fs_group(mut self, fs_group: ::models::V1beta1FsGroupStrategyOptions) -> V1beta1PodSecurityPolicySpec {
    self.fs_group = fs_group;
    self
  }

  pub fn fs_group(&self) -> &::models::V1beta1FsGroupStrategyOptions {
    &self.fs_group
  }


  pub fn set_host_ipc(&mut self, host_ipc: bool) {
    self.host_ipc = Some(host_ipc);
  }

  pub fn with_host_ipc(mut self, host_ipc: bool) -> V1beta1PodSecurityPolicySpec {
    self.host_ipc = Some(host_ipc);
    self
  }

  pub fn host_ipc(&self) -> Option<&bool> {
    self.host_ipc.as_ref()
  }

  pub fn reset_host_ipc(&mut self) {
    self.host_ipc = None;
  }

  pub fn set_host_network(&mut self, host_network: bool) {
    self.host_network = Some(host_network);
  }

  pub fn with_host_network(mut self, host_network: bool) -> V1beta1PodSecurityPolicySpec {
    self.host_network = Some(host_network);
    self
  }

  pub fn host_network(&self) -> Option<&bool> {
    self.host_network.as_ref()
  }

  pub fn reset_host_network(&mut self) {
    self.host_network = None;
  }

  pub fn set_host_pid(&mut self, host_pid: bool) {
    self.host_pid = Some(host_pid);
  }

  pub fn with_host_pid(mut self, host_pid: bool) -> V1beta1PodSecurityPolicySpec {
    self.host_pid = Some(host_pid);
    self
  }

  pub fn host_pid(&self) -> Option<&bool> {
    self.host_pid.as_ref()
  }

  pub fn reset_host_pid(&mut self) {
    self.host_pid = None;
  }

  pub fn set_host_ports(&mut self, host_ports: Vec<::models::V1beta1HostPortRange>) {
    self.host_ports = Some(host_ports);
  }

  pub fn with_host_ports(mut self, host_ports: Vec<::models::V1beta1HostPortRange>) -> V1beta1PodSecurityPolicySpec {
    self.host_ports = Some(host_ports);
    self
  }

  pub fn host_ports(&self) -> Option<&Vec<::models::V1beta1HostPortRange>> {
    self.host_ports.as_ref()
  }

  pub fn reset_host_ports(&mut self) {
    self.host_ports = None;
  }

  pub fn set_privileged(&mut self, privileged: bool) {
    self.privileged = Some(privileged);
  }

  pub fn with_privileged(mut self, privileged: bool) -> V1beta1PodSecurityPolicySpec {
    self.privileged = Some(privileged);
    self
  }

  pub fn privileged(&self) -> Option<&bool> {
    self.privileged.as_ref()
  }

  pub fn reset_privileged(&mut self) {
    self.privileged = None;
  }

  pub fn set_read_only_root_filesystem(&mut self, read_only_root_filesystem: bool) {
    self.read_only_root_filesystem = Some(read_only_root_filesystem);
  }

  pub fn with_read_only_root_filesystem(mut self, read_only_root_filesystem: bool) -> V1beta1PodSecurityPolicySpec {
    self.read_only_root_filesystem = Some(read_only_root_filesystem);
    self
  }

  pub fn read_only_root_filesystem(&self) -> Option<&bool> {
    self.read_only_root_filesystem.as_ref()
  }

  pub fn reset_read_only_root_filesystem(&mut self) {
    self.read_only_root_filesystem = None;
  }

  pub fn set_required_drop_capabilities(&mut self, required_drop_capabilities: Vec<String>) {
    self.required_drop_capabilities = Some(required_drop_capabilities);
  }

  pub fn with_required_drop_capabilities(mut self, required_drop_capabilities: Vec<String>) -> V1beta1PodSecurityPolicySpec {
    self.required_drop_capabilities = Some(required_drop_capabilities);
    self
  }

  pub fn required_drop_capabilities(&self) -> Option<&Vec<String>> {
    self.required_drop_capabilities.as_ref()
  }

  pub fn reset_required_drop_capabilities(&mut self) {
    self.required_drop_capabilities = None;
  }

  pub fn set_run_as_user(&mut self, run_as_user: ::models::V1beta1RunAsUserStrategyOptions) {
    self.run_as_user = run_as_user;
  }

  pub fn with_run_as_user(mut self, run_as_user: ::models::V1beta1RunAsUserStrategyOptions) -> V1beta1PodSecurityPolicySpec {
    self.run_as_user = run_as_user;
    self
  }

  pub fn run_as_user(&self) -> &::models::V1beta1RunAsUserStrategyOptions {
    &self.run_as_user
  }


  pub fn set_se_linux(&mut self, se_linux: ::models::V1beta1SeLinuxStrategyOptions) {
    self.se_linux = se_linux;
  }

  pub fn with_se_linux(mut self, se_linux: ::models::V1beta1SeLinuxStrategyOptions) -> V1beta1PodSecurityPolicySpec {
    self.se_linux = se_linux;
    self
  }

  pub fn se_linux(&self) -> &::models::V1beta1SeLinuxStrategyOptions {
    &self.se_linux
  }


  pub fn set_supplemental_groups(&mut self, supplemental_groups: ::models::V1beta1SupplementalGroupsStrategyOptions) {
    self.supplemental_groups = supplemental_groups;
  }

  pub fn with_supplemental_groups(mut self, supplemental_groups: ::models::V1beta1SupplementalGroupsStrategyOptions) -> V1beta1PodSecurityPolicySpec {
    self.supplemental_groups = supplemental_groups;
    self
  }

  pub fn supplemental_groups(&self) -> &::models::V1beta1SupplementalGroupsStrategyOptions {
    &self.supplemental_groups
  }


  pub fn set_volumes(&mut self, volumes: Vec<String>) {
    self.volumes = Some(volumes);
  }

  pub fn with_volumes(mut self, volumes: Vec<String>) -> V1beta1PodSecurityPolicySpec {
    self.volumes = Some(volumes);
    self
  }

  pub fn volumes(&self) -> Option<&Vec<String>> {
    self.volumes.as_ref()
  }

  pub fn reset_volumes(&mut self) {
    self.volumes = None;
  }

}


