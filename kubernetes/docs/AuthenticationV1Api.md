# \AuthenticationV1Api

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_authentication_v1_token_review**](AuthenticationV1Api.md#create_authentication_v1_token_review) | **Post** /apis/authentication.k8s.io/v1/tokenreviews | 
[**get_authentication_v1_api_resources**](AuthenticationV1Api.md#get_authentication_v1_api_resources) | **Get** /apis/authentication.k8s.io/v1/ | 


# **create_authentication_v1_token_review**
> ::models::IoK8sKubernetesPkgApisAuthenticationV1TokenReview create_authentication_v1_token_review(ctx, body, optional)


create a TokenReview

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**IoK8sKubernetesPkgApisAuthenticationV1TokenReview**](IoK8sKubernetesPkgApisAuthenticationV1TokenReview.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**IoK8sKubernetesPkgApisAuthenticationV1TokenReview**](IoK8sKubernetesPkgApisAuthenticationV1TokenReview.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisAuthenticationV1TokenReview**](io.k8s.kubernetes.pkg.apis.authentication.v1.TokenReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_authentication_v1_api_resources**
> ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList get_authentication_v1_api_resources(ctx, )


get available resources

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList**](io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json, application/yaml, application/vnd.kubernetes.protobuf
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

