# \CustomObjectsApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cluster_custom_object**](CustomObjectsApi.md#create_cluster_custom_object) | **Post** /apis/{group}/{version}/{plural} | 
[**create_namespaced_custom_object**](CustomObjectsApi.md#create_namespaced_custom_object) | **Post** /apis/{group}/{version}/namespaces/{namespace}/{plural} | 
[**delete_cluster_custom_object**](CustomObjectsApi.md#delete_cluster_custom_object) | **Delete** /apis/{group}/{version}/{plural}/{name} | 
[**delete_namespaced_custom_object**](CustomObjectsApi.md#delete_namespaced_custom_object) | **Delete** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name} | 
[**get_cluster_custom_object**](CustomObjectsApi.md#get_cluster_custom_object) | **Get** /apis/{group}/{version}/{plural}/{name} | 
[**get_cluster_custom_object_scale**](CustomObjectsApi.md#get_cluster_custom_object_scale) | **Get** /apis/{group}/{version}/{plural}/{name}/scale | 
[**get_cluster_custom_object_status**](CustomObjectsApi.md#get_cluster_custom_object_status) | **Get** /apis/{group}/{version}/{plural}/{name}/status | 
[**get_namespaced_custom_object**](CustomObjectsApi.md#get_namespaced_custom_object) | **Get** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name} | 
[**get_namespaced_custom_object_scale**](CustomObjectsApi.md#get_namespaced_custom_object_scale) | **Get** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/scale | 
[**get_namespaced_custom_object_status**](CustomObjectsApi.md#get_namespaced_custom_object_status) | **Get** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/status | 
[**list_cluster_custom_object**](CustomObjectsApi.md#list_cluster_custom_object) | **Get** /apis/{group}/{version}/{plural} | 
[**list_namespaced_custom_object**](CustomObjectsApi.md#list_namespaced_custom_object) | **Get** /apis/{group}/{version}/namespaces/{namespace}/{plural} | 
[**patch_cluster_custom_object**](CustomObjectsApi.md#patch_cluster_custom_object) | **Patch** /apis/{group}/{version}/{plural}/{name} | 
[**patch_cluster_custom_object_scale**](CustomObjectsApi.md#patch_cluster_custom_object_scale) | **Patch** /apis/{group}/{version}/{plural}/{name}/scale | 
[**patch_cluster_custom_object_status**](CustomObjectsApi.md#patch_cluster_custom_object_status) | **Patch** /apis/{group}/{version}/{plural}/{name}/status | 
[**patch_namespaced_custom_object**](CustomObjectsApi.md#patch_namespaced_custom_object) | **Patch** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name} | 
[**patch_namespaced_custom_object_scale**](CustomObjectsApi.md#patch_namespaced_custom_object_scale) | **Patch** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/scale | 
[**patch_namespaced_custom_object_status**](CustomObjectsApi.md#patch_namespaced_custom_object_status) | **Patch** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/status | 
[**replace_cluster_custom_object**](CustomObjectsApi.md#replace_cluster_custom_object) | **Put** /apis/{group}/{version}/{plural}/{name} | 
[**replace_cluster_custom_object_scale**](CustomObjectsApi.md#replace_cluster_custom_object_scale) | **Put** /apis/{group}/{version}/{plural}/{name}/scale | 
[**replace_cluster_custom_object_status**](CustomObjectsApi.md#replace_cluster_custom_object_status) | **Put** /apis/{group}/{version}/{plural}/{name}/status | 
[**replace_namespaced_custom_object**](CustomObjectsApi.md#replace_namespaced_custom_object) | **Put** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name} | 
[**replace_namespaced_custom_object_scale**](CustomObjectsApi.md#replace_namespaced_custom_object_scale) | **Put** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/scale | 
[**replace_namespaced_custom_object_status**](CustomObjectsApi.md#replace_namespaced_custom_object_status) | **Put** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/status | 


# **create_cluster_custom_object**
> Value create_cluster_custom_object(ctx, group, version, plural, body, optional)


Creates a cluster scoped Custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| The custom resource&#39;s group name | 
  **version** | **String**| The custom resource&#39;s version | 
  **plural** | **String**| The custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **body** | [**Value**](Value.md)| The JSON schema of the Resource to create. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group** | **String**| The custom resource&#39;s group name | 
 **version** | **String**| The custom resource&#39;s version | 
 **plural** | **String**| The custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
 **body** | [**Value**](Value.md)| The JSON schema of the Resource to create. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_namespaced_custom_object**
> Value create_namespaced_custom_object(ctx, group, version, namespace, plural, body, optional)


Creates a namespace scoped Custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| The custom resource&#39;s group name | 
  **version** | **String**| The custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| The custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **body** | [**Value**](Value.md)| The JSON schema of the Resource to create. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group** | **String**| The custom resource&#39;s group name | 
 **version** | **String**| The custom resource&#39;s version | 
 **namespace** | **String**| The custom resource&#39;s namespace | 
 **plural** | **String**| The custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
 **body** | [**Value**](Value.md)| The JSON schema of the Resource to create. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_cluster_custom_object**
> Value delete_cluster_custom_object(ctx, group, version, plural, name, body, optional)


Deletes the specified cluster scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **plural** | **String**| the custom object&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**V1DeleteOptions**](V1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group** | **String**| the custom resource&#39;s group | 
 **version** | **String**| the custom resource&#39;s version | 
 **plural** | **String**| the custom object&#39;s plural name. For TPRs this would be lowercase plural kind. | 
 **name** | **String**| the custom object&#39;s name | 
 **body** | [**V1DeleteOptions**](V1DeleteOptions.md)|  | 
 **grace_period_seconds** | **i32**| The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. | 
 **orphan_dependents** | **bool**| Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \&quot;orphan\&quot; finalizer will be added to/removed from the object&#39;s finalizers list. Either this field or PropagationPolicy may be set, but not both. | 
 **propagation_policy** | **String**| Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_namespaced_custom_object**
> Value delete_namespaced_custom_object(ctx, group, version, namespace, plural, name, body, optional)


Deletes the specified namespace scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**V1DeleteOptions**](V1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group** | **String**| the custom resource&#39;s group | 
 **version** | **String**| the custom resource&#39;s version | 
 **namespace** | **String**| The custom resource&#39;s namespace | 
 **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
 **name** | **String**| the custom object&#39;s name | 
 **body** | [**V1DeleteOptions**](V1DeleteOptions.md)|  | 
 **grace_period_seconds** | **i32**| The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. | 
 **orphan_dependents** | **bool**| Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \&quot;orphan\&quot; finalizer will be added to/removed from the object&#39;s finalizers list. Either this field or PropagationPolicy may be set, but not both. | 
 **propagation_policy** | **String**| Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_custom_object**
> Value get_cluster_custom_object(ctx, group, version, plural, name)


Returns a cluster scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **plural** | **String**| the custom object&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_custom_object_scale**
> Value get_cluster_custom_object_scale(ctx, group, version, plural, name)


read scale of the specified custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_custom_object_status**
> Value get_cluster_custom_object_status(ctx, group, version, plural, name)


read status of the specified cluster scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_namespaced_custom_object**
> Value get_namespaced_custom_object(ctx, group, version, namespace, plural, name)


Returns a namespace scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_namespaced_custom_object_scale**
> Value get_namespaced_custom_object_scale(ctx, group, version, namespace, plural, name)


read scale of the specified namespace scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_namespaced_custom_object_status**
> Value get_namespaced_custom_object_status(ctx, group, version, namespace, plural, name)


read status of the specified namespace scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_cluster_custom_object**
> Value list_cluster_custom_object(ctx, group, version, plural, optional)


list or watch cluster scoped custom objects

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| The custom resource&#39;s group name | 
  **version** | **String**| The custom resource&#39;s version | 
  **plural** | **String**| The custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group** | **String**| The custom resource&#39;s group name | 
 **version** | **String**| The custom resource&#39;s version | 
 **plural** | **String**| The custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/json;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_namespaced_custom_object**
> Value list_namespaced_custom_object(ctx, group, version, namespace, plural, optional)


list or watch namespace scoped custom objects

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| The custom resource&#39;s group name | 
  **version** | **String**| The custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| The custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group** | **String**| The custom resource&#39;s group name | 
 **version** | **String**| The custom resource&#39;s version | 
 **namespace** | **String**| The custom resource&#39;s namespace | 
 **plural** | **String**| The custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/json;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_cluster_custom_object**
> Value patch_cluster_custom_object(ctx, group, version, plural, name, body)


patch the specified cluster scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **plural** | **String**| the custom object&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)| The JSON schema of the Resource to patch. | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/merge-patch+json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_cluster_custom_object_scale**
> Value patch_cluster_custom_object_scale(ctx, group, version, plural, name, body)


partially update scale of the specified cluster scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_cluster_custom_object_status**
> Value patch_cluster_custom_object_status(ctx, group, version, plural, name, body)


partially update status of the specified cluster scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_namespaced_custom_object**
> Value patch_namespaced_custom_object(ctx, group, version, namespace, plural, name, body)


patch the specified namespace scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)| The JSON schema of the Resource to patch. | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/merge-patch+json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_namespaced_custom_object_scale**
> Value patch_namespaced_custom_object_scale(ctx, group, version, namespace, plural, name, body)


partially update scale of the specified namespace scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_namespaced_custom_object_status**
> Value patch_namespaced_custom_object_status(ctx, group, version, namespace, plural, name, body)


partially update status of the specified namespace scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_cluster_custom_object**
> Value replace_cluster_custom_object(ctx, group, version, plural, name, body)


replace the specified cluster scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **plural** | **String**| the custom object&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)| The JSON schema of the Resource to replace. | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_cluster_custom_object_scale**
> Value replace_cluster_custom_object_scale(ctx, group, version, plural, name, body)


replace scale of the specified cluster scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_cluster_custom_object_status**
> Value replace_cluster_custom_object_status(ctx, group, version, plural, name, body)


replace status of the cluster scoped specified custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_namespaced_custom_object**
> Value replace_namespaced_custom_object(ctx, group, version, namespace, plural, name, body)


replace the specified namespace scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)| The JSON schema of the Resource to replace. | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_namespaced_custom_object_scale**
> Value replace_namespaced_custom_object_scale(ctx, group, version, namespace, plural, name, body)


replace scale of the specified namespace scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_namespaced_custom_object_status**
> Value replace_namespaced_custom_object_status(ctx, group, version, namespace, plural, name, body)


replace status of the specified namespace scoped custom object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | **String**| the custom resource&#39;s group | 
  **version** | **String**| the custom resource&#39;s version | 
  **namespace** | **String**| The custom resource&#39;s namespace | 
  **plural** | **String**| the custom resource&#39;s plural name. For TPRs this would be lowercase plural kind. | 
  **name** | **String**| the custom object&#39;s name | 
  **body** | [**Value**](Value.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

