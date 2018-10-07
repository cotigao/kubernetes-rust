# IoK8sKubernetesPkgApiV1HttpGetAction

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | **String** | Host name to connect to, defaults to the pod IP. You probably want to set \&quot;Host\&quot; in httpHeaders instead. | [optional] 
**http_headers** | [**Vec<::models::IoK8sKubernetesPkgApiV1HttpHeader>**](io.k8s.kubernetes.pkg.api.v1.HTTPHeader.md) | Custom headers to set in the request. HTTP allows repeated headers. | [optional] 
**path** | **String** | Path to access on the HTTP server. | [optional] 
**port** | **String** |  | 
**scheme** | **String** | Scheme to use for connecting to the host. Defaults to HTTP. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


