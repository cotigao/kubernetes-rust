/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct AuthenticationV1ApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AuthenticationV1ApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AuthenticationV1ApiClient<C> {
        AuthenticationV1ApiClient {
            configuration: configuration,
        }
    }
}

pub trait AuthenticationV1Api {
    fn create_authentication_v1_token_review(&self, body: ::models::IoK8sKubernetesPkgApisAuthenticationV1TokenReview, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisAuthenticationV1TokenReview, Error = Error>>;
    fn get_authentication_v1_api_resources(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, Error = Error>>;
}


impl<C: hyper::client::Connect>AuthenticationV1Api for AuthenticationV1ApiClient<C> {
    fn create_authentication_v1_token_review(&self, body: ::models::IoK8sKubernetesPkgApisAuthenticationV1TokenReview, pretty: &str) -> Box<Future<Item = ::models::IoK8sKubernetesPkgApisAuthenticationV1TokenReview, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/authentication.k8s.io/v1/tokenreviews{}", configuration.base_path, query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::IoK8sKubernetesPkgApisAuthenticationV1TokenReview, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_authentication_v1_api_resources(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/apis/authentication.k8s.io/v1/", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}