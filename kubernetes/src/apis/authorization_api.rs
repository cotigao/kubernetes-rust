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

pub struct AuthorizationApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AuthorizationApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AuthorizationApiClient<C> {
        AuthorizationApiClient {
            configuration: configuration,
        }
    }
}

pub trait AuthorizationApi {
    fn get_authorization_api_group(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiGroup, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>AuthorizationApi for AuthorizationApiClient<C> {
    fn get_authorization_api_group(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiGroup, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/authorization.k8s.io/".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .execute(self.configuration.borrow())
    }

}
