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

pub struct StorageV1ApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> StorageV1ApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> StorageV1ApiClient<C> {
        StorageV1ApiClient {
            configuration: configuration,
        }
    }
}

pub trait StorageV1Api {
    fn create_storage_v1_storage_class(&self, io_k8s_kubernetes_pkg_apis_storage_v1_storage_class: ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, Error = Error<serde_json::Value>>>;
    fn delete_storage_v1_collection_storage_class(&self, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1Status, Error = Error<serde_json::Value>>>;
    fn delete_storage_v1_storage_class(&self, name: &str, io_k8s_apimachinery_pkg_apis_meta_v1_delete_options: ::models::IoK8sApimachineryPkgApisMetaV1DeleteOptions, pretty: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1Status, Error = Error<serde_json::Value>>>;
    fn get_storage_v1_api_resources(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, Error = Error<serde_json::Value>>>;
    fn list_storage_v1_storage_class(&self, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisStorageV1StorageClassList, Error = Error<serde_json::Value>>>;
    fn patch_storage_v1_storage_class(&self, name: &str, io_k8s_apimachinery_pkg_apis_meta_v1_patch: ::models::IoK8sApimachineryPkgApisMetaV1Patch, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, Error = Error<serde_json::Value>>>;
    fn read_storage_v1_storage_class(&self, name: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, Error = Error<serde_json::Value>>>;
    fn replace_storage_v1_storage_class(&self, name: &str, io_k8s_kubernetes_pkg_apis_storage_v1_storage_class: ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, Error = Error<serde_json::Value>>>;
    fn watch_storage_v1_storage_class(&self, name: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error = Error<serde_json::Value>>>;
    fn watch_storage_v1_storage_class_list(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>StorageV1Api for StorageV1ApiClient<C> {
    fn create_storage_v1_storage_class(&self, io_k8s_kubernetes_pkg_apis_storage_v1_storage_class: ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/apis/storage.k8s.io/v1/storageclasses".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_body_param(io_k8s_kubernetes_pkg_apis_storage_v1_storage_class)
            .execute(self.configuration.borrow())
    }

    fn delete_storage_v1_collection_storage_class(&self, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1Status, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/apis/storage.k8s.io/v1/storageclasses".to_string())
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
            .execute(self.configuration.borrow())
    }

    fn delete_storage_v1_storage_class(&self, name: &str, io_k8s_apimachinery_pkg_apis_meta_v1_delete_options: ::models::IoK8sApimachineryPkgApisMetaV1DeleteOptions, pretty: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1Status, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/apis/storage.k8s.io/v1/storageclasses/{name}".to_string())
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
            .with_body_param(io_k8s_apimachinery_pkg_apis_meta_v1_delete_options)
            .execute(self.configuration.borrow())
    }

    fn get_storage_v1_api_resources(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/storage.k8s.io/v1/".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .execute(self.configuration.borrow())
    }

    fn list_storage_v1_storage_class(&self, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisStorageV1StorageClassList, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/storage.k8s.io/v1/storageclasses".to_string())
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
            .execute(self.configuration.borrow())
    }

    fn patch_storage_v1_storage_class(&self, name: &str, io_k8s_apimachinery_pkg_apis_meta_v1_patch: ::models::IoK8sApimachineryPkgApisMetaV1Patch, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Patch, "/apis/storage.k8s.io/v1/storageclasses/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_path_param("name".to_string(), name.to_string())
            .with_body_param(io_k8s_apimachinery_pkg_apis_meta_v1_patch)
            .execute(self.configuration.borrow())
    }

    fn read_storage_v1_storage_class(&self, name: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/storage.k8s.io/v1/storageclasses/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_query_param("exact".to_string(), exact.to_string())
            .with_query_param("export".to_string(), export.to_string())
            .with_path_param("name".to_string(), name.to_string())
            .execute(self.configuration.borrow())
    }

    fn replace_storage_v1_storage_class(&self, name: &str, io_k8s_kubernetes_pkg_apis_storage_v1_storage_class: ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisStorageV1StorageClass, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/apis/storage.k8s.io/v1/storageclasses/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .with_query_param("pretty".to_string(), pretty.to_string())
            .with_path_param("name".to_string(), name.to_string())
            .with_body_param(io_k8s_kubernetes_pkg_apis_storage_v1_storage_class)
            .execute(self.configuration.borrow())
    }

    fn watch_storage_v1_storage_class(&self, name: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/storage.k8s.io/v1/watch/storageclasses/{name}".to_string())
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
            .execute(self.configuration.borrow())
    }

    fn watch_storage_v1_storage_class_list(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/storage.k8s.io/v1/watch/storageclasses".to_string())
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
