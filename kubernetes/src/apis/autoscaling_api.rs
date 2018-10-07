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

pub struct AutoscalingApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AutoscalingApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AutoscalingApiClient<C> {
        AutoscalingApiClient {
            configuration: configuration,
        }
    }
}

pub trait AutoscalingApi {
    fn get_autoscaling_api_group(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiGroup, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>AutoscalingApi for AutoscalingApiClient<C> {
    fn get_autoscaling_api_group(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiGroup, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/apis/autoscaling/".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
            .execute(self.configuration.borrow())
    }

}
