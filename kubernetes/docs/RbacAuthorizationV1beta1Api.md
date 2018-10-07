# \RbacAuthorizationV1beta1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_rbac_authorization_v1beta1_cluster_role**](RbacAuthorizationV1beta1Api.md#create_rbac_authorization_v1beta1_cluster_role) | **Post** /apis/rbac.authorization.k8s.io/v1beta1/clusterroles | 
[**create_rbac_authorization_v1beta1_cluster_role_binding**](RbacAuthorizationV1beta1Api.md#create_rbac_authorization_v1beta1_cluster_role_binding) | **Post** /apis/rbac.authorization.k8s.io/v1beta1/clusterrolebindings | 
[**create_rbac_authorization_v1beta1_namespaced_role**](RbacAuthorizationV1beta1Api.md#create_rbac_authorization_v1beta1_namespaced_role) | **Post** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/roles | 
[**create_rbac_authorization_v1beta1_namespaced_role_binding**](RbacAuthorizationV1beta1Api.md#create_rbac_authorization_v1beta1_namespaced_role_binding) | **Post** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings | 
[**delete_rbac_authorization_v1beta1_cluster_role**](RbacAuthorizationV1beta1Api.md#delete_rbac_authorization_v1beta1_cluster_role) | **Delete** /apis/rbac.authorization.k8s.io/v1beta1/clusterroles/{name} | 
[**delete_rbac_authorization_v1beta1_cluster_role_binding**](RbacAuthorizationV1beta1Api.md#delete_rbac_authorization_v1beta1_cluster_role_binding) | **Delete** /apis/rbac.authorization.k8s.io/v1beta1/clusterrolebindings/{name} | 
[**delete_rbac_authorization_v1beta1_collection_cluster_role**](RbacAuthorizationV1beta1Api.md#delete_rbac_authorization_v1beta1_collection_cluster_role) | **Delete** /apis/rbac.authorization.k8s.io/v1beta1/clusterroles | 
[**delete_rbac_authorization_v1beta1_collection_cluster_role_binding**](RbacAuthorizationV1beta1Api.md#delete_rbac_authorization_v1beta1_collection_cluster_role_binding) | **Delete** /apis/rbac.authorization.k8s.io/v1beta1/clusterrolebindings | 
[**delete_rbac_authorization_v1beta1_collection_namespaced_role**](RbacAuthorizationV1beta1Api.md#delete_rbac_authorization_v1beta1_collection_namespaced_role) | **Delete** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/roles | 
[**delete_rbac_authorization_v1beta1_collection_namespaced_role_binding**](RbacAuthorizationV1beta1Api.md#delete_rbac_authorization_v1beta1_collection_namespaced_role_binding) | **Delete** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings | 
[**delete_rbac_authorization_v1beta1_namespaced_role**](RbacAuthorizationV1beta1Api.md#delete_rbac_authorization_v1beta1_namespaced_role) | **Delete** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/roles/{name} | 
[**delete_rbac_authorization_v1beta1_namespaced_role_binding**](RbacAuthorizationV1beta1Api.md#delete_rbac_authorization_v1beta1_namespaced_role_binding) | **Delete** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings/{name} | 
[**get_rbac_authorization_v1beta1_api_resources**](RbacAuthorizationV1beta1Api.md#get_rbac_authorization_v1beta1_api_resources) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/ | 
[**list_rbac_authorization_v1beta1_cluster_role**](RbacAuthorizationV1beta1Api.md#list_rbac_authorization_v1beta1_cluster_role) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/clusterroles | 
[**list_rbac_authorization_v1beta1_cluster_role_binding**](RbacAuthorizationV1beta1Api.md#list_rbac_authorization_v1beta1_cluster_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/clusterrolebindings | 
[**list_rbac_authorization_v1beta1_namespaced_role**](RbacAuthorizationV1beta1Api.md#list_rbac_authorization_v1beta1_namespaced_role) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/roles | 
[**list_rbac_authorization_v1beta1_namespaced_role_binding**](RbacAuthorizationV1beta1Api.md#list_rbac_authorization_v1beta1_namespaced_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings | 
[**list_rbac_authorization_v1beta1_role_binding_for_all_namespaces**](RbacAuthorizationV1beta1Api.md#list_rbac_authorization_v1beta1_role_binding_for_all_namespaces) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/rolebindings | 
[**list_rbac_authorization_v1beta1_role_for_all_namespaces**](RbacAuthorizationV1beta1Api.md#list_rbac_authorization_v1beta1_role_for_all_namespaces) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/roles | 
[**patch_rbac_authorization_v1beta1_cluster_role**](RbacAuthorizationV1beta1Api.md#patch_rbac_authorization_v1beta1_cluster_role) | **Patch** /apis/rbac.authorization.k8s.io/v1beta1/clusterroles/{name} | 
[**patch_rbac_authorization_v1beta1_cluster_role_binding**](RbacAuthorizationV1beta1Api.md#patch_rbac_authorization_v1beta1_cluster_role_binding) | **Patch** /apis/rbac.authorization.k8s.io/v1beta1/clusterrolebindings/{name} | 
[**patch_rbac_authorization_v1beta1_namespaced_role**](RbacAuthorizationV1beta1Api.md#patch_rbac_authorization_v1beta1_namespaced_role) | **Patch** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/roles/{name} | 
[**patch_rbac_authorization_v1beta1_namespaced_role_binding**](RbacAuthorizationV1beta1Api.md#patch_rbac_authorization_v1beta1_namespaced_role_binding) | **Patch** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings/{name} | 
[**read_rbac_authorization_v1beta1_cluster_role**](RbacAuthorizationV1beta1Api.md#read_rbac_authorization_v1beta1_cluster_role) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/clusterroles/{name} | 
[**read_rbac_authorization_v1beta1_cluster_role_binding**](RbacAuthorizationV1beta1Api.md#read_rbac_authorization_v1beta1_cluster_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/clusterrolebindings/{name} | 
[**read_rbac_authorization_v1beta1_namespaced_role**](RbacAuthorizationV1beta1Api.md#read_rbac_authorization_v1beta1_namespaced_role) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/roles/{name} | 
[**read_rbac_authorization_v1beta1_namespaced_role_binding**](RbacAuthorizationV1beta1Api.md#read_rbac_authorization_v1beta1_namespaced_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings/{name} | 
[**replace_rbac_authorization_v1beta1_cluster_role**](RbacAuthorizationV1beta1Api.md#replace_rbac_authorization_v1beta1_cluster_role) | **Put** /apis/rbac.authorization.k8s.io/v1beta1/clusterroles/{name} | 
[**replace_rbac_authorization_v1beta1_cluster_role_binding**](RbacAuthorizationV1beta1Api.md#replace_rbac_authorization_v1beta1_cluster_role_binding) | **Put** /apis/rbac.authorization.k8s.io/v1beta1/clusterrolebindings/{name} | 
[**replace_rbac_authorization_v1beta1_namespaced_role**](RbacAuthorizationV1beta1Api.md#replace_rbac_authorization_v1beta1_namespaced_role) | **Put** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/roles/{name} | 
[**replace_rbac_authorization_v1beta1_namespaced_role_binding**](RbacAuthorizationV1beta1Api.md#replace_rbac_authorization_v1beta1_namespaced_role_binding) | **Put** /apis/rbac.authorization.k8s.io/v1beta1/namespaces/{namespace}/rolebindings/{name} | 
[**watch_rbac_authorization_v1beta1_cluster_role**](RbacAuthorizationV1beta1Api.md#watch_rbac_authorization_v1beta1_cluster_role) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/watch/clusterroles/{name} | 
[**watch_rbac_authorization_v1beta1_cluster_role_binding**](RbacAuthorizationV1beta1Api.md#watch_rbac_authorization_v1beta1_cluster_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/watch/clusterrolebindings/{name} | 
[**watch_rbac_authorization_v1beta1_cluster_role_binding_list**](RbacAuthorizationV1beta1Api.md#watch_rbac_authorization_v1beta1_cluster_role_binding_list) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/watch/clusterrolebindings | 
[**watch_rbac_authorization_v1beta1_cluster_role_list**](RbacAuthorizationV1beta1Api.md#watch_rbac_authorization_v1beta1_cluster_role_list) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/watch/clusterroles | 
[**watch_rbac_authorization_v1beta1_namespaced_role**](RbacAuthorizationV1beta1Api.md#watch_rbac_authorization_v1beta1_namespaced_role) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/watch/namespaces/{namespace}/roles/{name} | 
[**watch_rbac_authorization_v1beta1_namespaced_role_binding**](RbacAuthorizationV1beta1Api.md#watch_rbac_authorization_v1beta1_namespaced_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/watch/namespaces/{namespace}/rolebindings/{name} | 
[**watch_rbac_authorization_v1beta1_namespaced_role_binding_list**](RbacAuthorizationV1beta1Api.md#watch_rbac_authorization_v1beta1_namespaced_role_binding_list) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/watch/namespaces/{namespace}/rolebindings | 
[**watch_rbac_authorization_v1beta1_namespaced_role_list**](RbacAuthorizationV1beta1Api.md#watch_rbac_authorization_v1beta1_namespaced_role_list) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/watch/namespaces/{namespace}/roles | 
[**watch_rbac_authorization_v1beta1_role_binding_list_for_all_namespaces**](RbacAuthorizationV1beta1Api.md#watch_rbac_authorization_v1beta1_role_binding_list_for_all_namespaces) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/watch/rolebindings | 
[**watch_rbac_authorization_v1beta1_role_list_for_all_namespaces**](RbacAuthorizationV1beta1Api.md#watch_rbac_authorization_v1beta1_role_list_for_all_namespaces) | **Get** /apis/rbac.authorization.k8s.io/v1beta1/watch/roles | 


# **create_rbac_authorization_v1beta1_cluster_role**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRole create_rbac_authorization_v1beta1_cluster_role(ctx, io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role, optional)


create a ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role** | [**IoK8sKubernetesPkgApisRbacV1beta1ClusterRole**](IoK8sKubernetesPkgApisRbacV1beta1ClusterRole.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role** | [**IoK8sKubernetesPkgApisRbacV1beta1ClusterRole**](IoK8sKubernetesPkgApisRbacV1beta1ClusterRole.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRole**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_rbac_authorization_v1beta1_cluster_role_binding**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding create_rbac_authorization_v1beta1_cluster_role_binding(ctx, io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role_binding, optional)


create a ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role_binding** | [**IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding**](IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role_binding** | [**IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding**](IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_rbac_authorization_v1beta1_namespaced_role**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1Role create_rbac_authorization_v1beta1_namespaced_role(ctx, namespace, io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role, optional)


create a Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role** | [**IoK8sKubernetesPkgApisRbacV1beta1Role**](IoK8sKubernetesPkgApisRbacV1beta1Role.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role** | [**IoK8sKubernetesPkgApisRbacV1beta1Role**](IoK8sKubernetesPkgApisRbacV1beta1Role.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1Role**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_rbac_authorization_v1beta1_namespaced_role_binding**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBinding create_rbac_authorization_v1beta1_namespaced_role_binding(ctx, namespace, io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role_binding, optional)


create a RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role_binding** | [**IoK8sKubernetesPkgApisRbacV1beta1RoleBinding**](IoK8sKubernetesPkgApisRbacV1beta1RoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role_binding** | [**IoK8sKubernetesPkgApisRbacV1beta1RoleBinding**](IoK8sKubernetesPkgApisRbacV1beta1RoleBinding.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rbac_authorization_v1beta1_cluster_role**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1beta1_cluster_role(ctx, name, io_k8s_apimachinery_pkg_apis_meta_v1_delete_options, optional)


delete a ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_delete_options** | [**IoK8sApimachineryPkgApisMetaV1DeleteOptions**](IoK8sApimachineryPkgApisMetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_delete_options** | [**IoK8sApimachineryPkgApisMetaV1DeleteOptions**](IoK8sApimachineryPkgApisMetaV1DeleteOptions.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **grace_period_seconds** | **i32**| The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. | 
 **orphan_dependents** | **bool**| Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \&quot;orphan\&quot; finalizer will be added to/removed from the object&#39;s finalizers list. Either this field or PropagationPolicy may be set, but not both. | 
 **propagation_policy** | **String**| Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rbac_authorization_v1beta1_cluster_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1beta1_cluster_role_binding(ctx, name, io_k8s_apimachinery_pkg_apis_meta_v1_delete_options, optional)


delete a ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_delete_options** | [**IoK8sApimachineryPkgApisMetaV1DeleteOptions**](IoK8sApimachineryPkgApisMetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_delete_options** | [**IoK8sApimachineryPkgApisMetaV1DeleteOptions**](IoK8sApimachineryPkgApisMetaV1DeleteOptions.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **grace_period_seconds** | **i32**| The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. | 
 **orphan_dependents** | **bool**| Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \&quot;orphan\&quot; finalizer will be added to/removed from the object&#39;s finalizers list. Either this field or PropagationPolicy may be set, but not both. | 
 **propagation_policy** | **String**| Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rbac_authorization_v1beta1_collection_cluster_role**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1beta1_collection_cluster_role(ctx, optional)


delete collection of ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rbac_authorization_v1beta1_collection_cluster_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1beta1_collection_cluster_role_binding(ctx, optional)


delete collection of ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rbac_authorization_v1beta1_collection_namespaced_role**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1beta1_collection_namespaced_role(ctx, namespace, optional)


delete collection of Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rbac_authorization_v1beta1_collection_namespaced_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1beta1_collection_namespaced_role_binding(ctx, namespace, optional)


delete collection of RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rbac_authorization_v1beta1_namespaced_role**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1beta1_namespaced_role(ctx, name, namespace, io_k8s_apimachinery_pkg_apis_meta_v1_delete_options, optional)


delete a Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_delete_options** | [**IoK8sApimachineryPkgApisMetaV1DeleteOptions**](IoK8sApimachineryPkgApisMetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_delete_options** | [**IoK8sApimachineryPkgApisMetaV1DeleteOptions**](IoK8sApimachineryPkgApisMetaV1DeleteOptions.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **grace_period_seconds** | **i32**| The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. | 
 **orphan_dependents** | **bool**| Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \&quot;orphan\&quot; finalizer will be added to/removed from the object&#39;s finalizers list. Either this field or PropagationPolicy may be set, but not both. | 
 **propagation_policy** | **String**| Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rbac_authorization_v1beta1_namespaced_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1Status delete_rbac_authorization_v1beta1_namespaced_role_binding(ctx, name, namespace, io_k8s_apimachinery_pkg_apis_meta_v1_delete_options, optional)


delete a RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_delete_options** | [**IoK8sApimachineryPkgApisMetaV1DeleteOptions**](IoK8sApimachineryPkgApisMetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_delete_options** | [**IoK8sApimachineryPkgApisMetaV1DeleteOptions**](IoK8sApimachineryPkgApisMetaV1DeleteOptions.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **grace_period_seconds** | **i32**| The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. | 
 **orphan_dependents** | **bool**| Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \&quot;orphan\&quot; finalizer will be added to/removed from the object&#39;s finalizers list. Either this field or PropagationPolicy may be set, but not both. | 
 **propagation_policy** | **String**| Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rbac_authorization_v1beta1_api_resources**
> ::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList get_rbac_authorization_v1beta1_api_resources(ctx, )


get available resources

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList**](io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1beta1_cluster_role**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleList list_rbac_authorization_v1beta1_cluster_role(ctx, optional)


list or watch objects of kind ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleList**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRoleList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1beta1_cluster_role_binding**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBindingList list_rbac_authorization_v1beta1_cluster_role_binding(ctx, optional)


list or watch objects of kind ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBindingList**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRoleBindingList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1beta1_namespaced_role**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleList list_rbac_authorization_v1beta1_namespaced_role(ctx, namespace, optional)


list or watch objects of kind Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1RoleList**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.RoleList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1beta1_namespaced_role_binding**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBindingList list_rbac_authorization_v1beta1_namespaced_role_binding(ctx, namespace, optional)


list or watch objects of kind RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBindingList**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.RoleBindingList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1beta1_role_binding_for_all_namespaces**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBindingList list_rbac_authorization_v1beta1_role_binding_for_all_namespaces(ctx, optional)


list or watch objects of kind RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBindingList**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.RoleBindingList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1beta1_role_for_all_namespaces**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleList list_rbac_authorization_v1beta1_role_for_all_namespaces(ctx, optional)


list or watch objects of kind Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1RoleList**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.RoleList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1beta1_cluster_role**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRole patch_rbac_authorization_v1beta1_cluster_role(ctx, name, io_k8s_apimachinery_pkg_apis_meta_v1_patch, optional)


partially update the specified ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRole**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1beta1_cluster_role_binding**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding patch_rbac_authorization_v1beta1_cluster_role_binding(ctx, name, io_k8s_apimachinery_pkg_apis_meta_v1_patch, optional)


partially update the specified ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1beta1_namespaced_role**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1Role patch_rbac_authorization_v1beta1_namespaced_role(ctx, name, namespace, io_k8s_apimachinery_pkg_apis_meta_v1_patch, optional)


partially update the specified Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1Role**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1beta1_namespaced_role_binding**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBinding patch_rbac_authorization_v1beta1_namespaced_role_binding(ctx, name, namespace, io_k8s_apimachinery_pkg_apis_meta_v1_patch, optional)


partially update the specified RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_apimachinery_pkg_apis_meta_v1_patch** | [**IoK8sApimachineryPkgApisMetaV1Patch**](IoK8sApimachineryPkgApisMetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1beta1_cluster_role**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRole read_rbac_authorization_v1beta1_cluster_role(ctx, name, optional)


read the specified ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRole**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1beta1_cluster_role_binding**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding read_rbac_authorization_v1beta1_cluster_role_binding(ctx, name, optional)


read the specified ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1beta1_namespaced_role**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1Role read_rbac_authorization_v1beta1_namespaced_role(ctx, name, namespace, optional)


read the specified Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1Role**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1beta1_namespaced_role_binding**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBinding read_rbac_authorization_v1beta1_namespaced_role_binding(ctx, name, namespace, optional)


read the specified RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1beta1_cluster_role**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRole replace_rbac_authorization_v1beta1_cluster_role(ctx, name, io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role, optional)


replace the specified ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
  **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role** | [**IoK8sKubernetesPkgApisRbacV1beta1ClusterRole**](IoK8sKubernetesPkgApisRbacV1beta1ClusterRole.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role** | [**IoK8sKubernetesPkgApisRbacV1beta1ClusterRole**](IoK8sKubernetesPkgApisRbacV1beta1ClusterRole.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRole**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1beta1_cluster_role_binding**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding replace_rbac_authorization_v1beta1_cluster_role_binding(ctx, name, io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role_binding, optional)


replace the specified ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
  **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role_binding** | [**IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding**](IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_cluster_role_binding** | [**IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding**](IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1ClusterRoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1beta1_namespaced_role**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1Role replace_rbac_authorization_v1beta1_namespaced_role(ctx, name, namespace, io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role, optional)


replace the specified Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role** | [**IoK8sKubernetesPkgApisRbacV1beta1Role**](IoK8sKubernetesPkgApisRbacV1beta1Role.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role** | [**IoK8sKubernetesPkgApisRbacV1beta1Role**](IoK8sKubernetesPkgApisRbacV1beta1Role.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1Role**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1beta1_namespaced_role_binding**
> ::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBinding replace_rbac_authorization_v1beta1_namespaced_role_binding(ctx, name, namespace, io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role_binding, optional)


replace the specified RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role_binding** | [**IoK8sKubernetesPkgApisRbacV1beta1RoleBinding**](IoK8sKubernetesPkgApisRbacV1beta1RoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **io_k8s_kubernetes_pkg_apis_rbac_v1beta1_role_binding** | [**IoK8sKubernetesPkgApisRbacV1beta1RoleBinding**](IoK8sKubernetesPkgApisRbacV1beta1RoleBinding.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::IoK8sKubernetesPkgApisRbacV1beta1RoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1beta1_cluster_role**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1beta1_cluster_role(ctx, name, optional)


watch changes to an object of kind ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1beta1_cluster_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1beta1_cluster_role_binding(ctx, name, optional)


watch changes to an object of kind ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1beta1_cluster_role_binding_list**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1beta1_cluster_role_binding_list(ctx, optional)


watch individual changes to a list of ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1beta1_cluster_role_list**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1beta1_cluster_role_list(ctx, optional)


watch individual changes to a list of ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1beta1_namespaced_role**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1beta1_namespaced_role(ctx, name, namespace, optional)


watch changes to an object of kind Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1beta1_namespaced_role_binding**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1beta1_namespaced_role_binding(ctx, name, namespace, optional)


watch changes to an object of kind RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1beta1_namespaced_role_binding_list**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1beta1_namespaced_role_binding_list(ctx, namespace, optional)


watch individual changes to a list of RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1beta1_namespaced_role_list**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1beta1_namespaced_role_list(ctx, namespace, optional)


watch individual changes to a list of Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1beta1_role_binding_list_for_all_namespaces**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1beta1_role_binding_list_for_all_namespaces(ctx, optional)


watch individual changes to a list of RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1beta1_role_list_for_all_namespaces**
> ::models::IoK8sApimachineryPkgApisMetaV1WatchEvent watch_rbac_authorization_v1beta1_role_list_for_all_namespaces(ctx, optional)


watch individual changes to a list of Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::IoK8sApimachineryPkgApisMetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

