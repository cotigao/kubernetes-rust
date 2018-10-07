/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct NetworkingV1ApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> NetworkingV1ApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> NetworkingV1ApiClient<C> {
        NetworkingV1ApiClient {
            configuration: configuration,
        }
    }
}

pub trait NetworkingV1Api {
    fn create_networking_v1_namespaced_network_policy(&self, namespace: &str, io_k8s_kubernetes_pkg_apis_networking_v1_network_policy: ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, Error = Error<serde_json::Value>>>;
    fn delete_networking_v1_collection_namespaced_network_policy(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1Status, Error = Error<serde_json::Value>>>;
    fn delete_networking_v1_namespaced_network_policy(&self, name: &str, namespace: &str, io_k8s_apimachinery_pkg_apis_meta_v1_delete_options: ::models::IoK8sApimachineryPkgApisMetaV1DeleteOptions, pretty: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1Status, Error = Error<serde_json::Value>>>;
    fn get_networking_v1_api_resources(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, Error = Error<serde_json::Value>>>;
    fn list_networking_v1_namespaced_network_policy(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyList, Error = Error<serde_json::Value>>>;
    fn list_networking_v1_network_policy_for_all_namespaces(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyList, Error = Error<serde_json::Value>>>;
    fn patch_networking_v1_namespaced_network_policy(&self, name: &str, namespace: &str, io_k8s_apimachinery_pkg_apis_meta_v1_patch: ::models::IoK8sApimachineryPkgApisMetaV1Patch, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, Error = Error<serde_json::Value>>>;
    fn read_networking_v1_namespaced_network_policy(&self, name: &str, namespace: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, Error = Error<serde_json::Value>>>;
    fn replace_networking_v1_namespaced_network_policy(&self, name: &str, namespace: &str, io_k8s_kubernetes_pkg_apis_networking_v1_network_policy: ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, Error = Error<serde_json::Value>>>;
    fn watch_networking_v1_namespaced_network_policy(&self, name: &str, namespace: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error = Error<serde_json::Value>>>;
    fn watch_networking_v1_namespaced_network_policy_list(&self, namespace: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error = Error<serde_json::Value>>>;
    fn watch_networking_v1_network_policy_list_for_all_namespaces(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>NetworkingV1Api for NetworkingV1ApiClient<C> {
    fn create_networking_v1_namespaced_network_policy(&self, namespace: &str, io_k8s_kubernetes_pkg_apis_networking_v1_network_policy: ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_body_param(io_k8s_kubernetes_pkg_apis_networking_v1_network_policy)
            .execute(self.configuration.borrow())
    }

    fn delete_networking_v1_collection_namespaced_network_policy(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1Status, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_query_param("fieldSelector".to_string(), field_selector.to_string())
            .with_query_param("includeUninitialized".to_string(), include_uninitialized.to_string())
            .with_query_param("labelSelector".to_string(), label_selector.to_string())
            .with_query_param("resourceVersion".to_string(), resource_version.to_string())
            .with_query_param("timeoutSeconds".to_string(), timeout_seconds.to_string())
            .with_query_param("watch".to_string(), watch.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .execute(self.configuration.borrow())
    }

    fn delete_networking_v1_namespaced_network_policy(&self, name: &str, namespace: &str, io_k8s_apimachinery_pkg_apis_meta_v1_delete_options: ::models::IoK8sApimachineryPkgApisMetaV1DeleteOptions, pretty: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1Status, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_query_param("gracePeriodSeconds".to_string(), grace_period_seconds.to_string())
            .with_query_param("orphanDependents".to_string(), orphan_dependents.to_string())
            .with_query_param("propagationPolicy".to_string(), propagation_policy.to_string())
            .with_path_param("name".to_string(), name.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_body_param(io_k8s_apimachinery_pkg_apis_meta_v1_delete_options)
            .execute(self.configuration.borrow())
    }

    fn get_networking_v1_api_resources(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/networking.k8s.io/v1/".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .execute(self.configuration.borrow())
    }

    fn list_networking_v1_namespaced_network_policy(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyList, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_query_param("fieldSelector".to_string(), field_selector.to_string())
            .with_query_param("includeUninitialized".to_string(), include_uninitialized.to_string())
            .with_query_param("labelSelector".to_string(), label_selector.to_string())
            .with_query_param("resourceVersion".to_string(), resource_version.to_string())
            .with_query_param("timeoutSeconds".to_string(), timeout_seconds.to_string())
            .with_query_param("watch".to_string(), watch.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .execute(self.configuration.borrow())
    }

    fn list_networking_v1_network_policy_for_all_namespaces(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyList, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/networking.k8s.io/v1/networkpolicies".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("fieldSelector".to_string(), field_selector.to_string())
            .with_query_param("includeUninitialized".to_string(), include_uninitialized.to_string())
            .with_query_param("labelSelector".to_string(), label_selector.to_string())
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_query_param("resourceVersion".to_string(), resource_version.to_string())
            .with_query_param("timeoutSeconds".to_string(), timeout_seconds.to_string())
            .with_query_param("watch".to_string(), watch.to_string())
            .execute(self.configuration.borrow())
    }

    fn patch_networking_v1_namespaced_network_policy(&self, name: &str, namespace: &str, io_k8s_apimachinery_pkg_apis_meta_v1_patch: ::models::IoK8sApimachineryPkgApisMetaV1Patch, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Patch, "/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_path_param("name".to_string(), name.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_body_param(io_k8s_apimachinery_pkg_apis_meta_v1_patch)
            .execute(self.configuration.borrow())
    }

    fn read_networking_v1_namespaced_network_policy(&self, name: &str, namespace: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_query_param("exact".to_string(), exact.to_string())
            .with_query_param("export".to_string(), export.to_string())
            .with_path_param("name".to_string(), name.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .execute(self.configuration.borrow())
    }

    fn replace_networking_v1_namespaced_network_policy(&self, name: &str, namespace: &str, io_k8s_kubernetes_pkg_apis_networking_v1_network_policy: ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicy, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_path_param("name".to_string(), name.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_body_param(io_k8s_kubernetes_pkg_apis_networking_v1_network_policy)
            .execute(self.configuration.borrow())
    }

    fn watch_networking_v1_namespaced_network_policy(&self, name: &str, namespace: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/networking.k8s.io/v1/watch/namespaces/{namespace}/networkpolicies/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("fieldSelector".to_string(), field_selector.to_string())
            .with_query_param("includeUninitialized".to_string(), include_uninitialized.to_string())
            .with_query_param("labelSelector".to_string(), label_selector.to_string())
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_query_param("resourceVersion".to_string(), resource_version.to_string())
            .with_query_param("timeoutSeconds".to_string(), timeout_seconds.to_string())
            .with_query_param("watch".to_string(), watch.to_string())
            .with_path_param("name".to_string(), name.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .execute(self.configuration.borrow())
    }

    fn watch_networking_v1_namespaced_network_policy_list(&self, namespace: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/networking.k8s.io/v1/watch/namespaces/{namespace}/networkpolicies".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("fieldSelector".to_string(), field_selector.to_string())
            .with_query_param("includeUninitialized".to_string(), include_uninitialized.to_string())
            .with_query_param("labelSelector".to_string(), label_selector.to_string())
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_query_param("resourceVersion".to_string(), resource_version.to_string())
            .with_query_param("timeoutSeconds".to_string(), timeout_seconds.to_string())
            .with_query_param("watch".to_string(), watch.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .execute(self.configuration.borrow())
    }

    fn watch_networking_v1_network_policy_list_for_all_namespaces(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/networking.k8s.io/v1/watch/networkpolicies".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("fieldSelector".to_string(), field_selector.to_string())
            .with_query_param("includeUninitialized".to_string(), include_uninitialized.to_string())
            .with_query_param("labelSelector".to_string(), label_selector.to_string())
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_query_param("resourceVersion".to_string(), resource_version.to_string())
            .with_query_param("timeoutSeconds".to_string(), timeout_seconds.to_string())
            .with_query_param("watch".to_string(), watch.to_string())
            .execute(self.configuration.borrow())
    }

}
