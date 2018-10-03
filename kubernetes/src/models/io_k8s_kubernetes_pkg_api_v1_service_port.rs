/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApiV1ServicePort : ServicePort contains information on service's port.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1ServicePort {
  /// The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. This maps to the 'Name' field in EndpointPort objects. Optional if only one ServicePort is defined on this service.
  #[serde(rename = "name")]
  name: Option<String>,
  /// The port on each node on which this service is exposed when type=NodePort or LoadBalancer. Usually assigned by the system. If specified, it will be allocated to the service if unused or else creation of the service will fail. Default is to auto-allocate a port if the ServiceType of this Service requires one. More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport
  #[serde(rename = "nodePort")]
  node_port: Option<i32>,
  /// The port that will be exposed by this service.
  #[serde(rename = "port")]
  port: i32,
  /// The IP protocol for this port. Supports \"TCP\" and \"UDP\". Default is TCP.
  #[serde(rename = "protocol")]
  protocol: Option<String>,
  /// Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service
  #[serde(rename = "targetPort")]
  target_port: Option<::models::IoK8sApimachineryPkgUtilIntstrIntOrString>
}

impl IoK8sKubernetesPkgApiV1ServicePort {
  /// ServicePort contains information on service's port.
  pub fn new(port: i32) -> IoK8sKubernetesPkgApiV1ServicePort {
    IoK8sKubernetesPkgApiV1ServicePort {
      name: None,
      node_port: None,
      port: port,
      protocol: None,
      target_port: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> IoK8sKubernetesPkgApiV1ServicePort {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_node_port(&mut self, node_port: i32) {
    self.node_port = Some(node_port);
  }

  pub fn with_node_port(mut self, node_port: i32) -> IoK8sKubernetesPkgApiV1ServicePort {
    self.node_port = Some(node_port);
    self
  }

  pub fn node_port(&self) -> Option<&i32> {
    self.node_port.as_ref()
  }

  pub fn reset_node_port(&mut self) {
    self.node_port = None;
  }

  pub fn set_port(&mut self, port: i32) {
    self.port = port;
  }

  pub fn with_port(mut self, port: i32) -> IoK8sKubernetesPkgApiV1ServicePort {
    self.port = port;
    self
  }

  pub fn port(&self) -> &i32 {
    &self.port
  }


  pub fn set_protocol(&mut self, protocol: String) {
    self.protocol = Some(protocol);
  }

  pub fn with_protocol(mut self, protocol: String) -> IoK8sKubernetesPkgApiV1ServicePort {
    self.protocol = Some(protocol);
    self
  }

  pub fn protocol(&self) -> Option<&String> {
    self.protocol.as_ref()
  }

  pub fn reset_protocol(&mut self) {
    self.protocol = None;
  }

  pub fn set_target_port(&mut self, target_port: ::models::IoK8sApimachineryPkgUtilIntstrIntOrString) {
    self.target_port = Some(target_port);
  }

  pub fn with_target_port(mut self, target_port: ::models::IoK8sApimachineryPkgUtilIntstrIntOrString) -> IoK8sKubernetesPkgApiV1ServicePort {
    self.target_port = Some(target_port);
    self
  }

  pub fn target_port(&self) -> Option<&::models::IoK8sApimachineryPkgUtilIntstrIntOrString> {
    self.target_port.as_ref()
  }

  pub fn reset_target_port(&mut self) {
    self.target_port = None;
  }

}


