# IoK8sKubernetesPkgApisRbacV1alpha1ClusterRoleBinding

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] 
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] 
**metadata** | [***::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta**](io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.md) |  | [optional] 
**role_ref** | [***::models::IoK8sKubernetesPkgApisRbacV1alpha1RoleRef**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.RoleRef.md) |  | 
**subjects** | [**Vec<::models::IoK8sKubernetesPkgApisRbacV1alpha1Subject>**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.Subject.md) | Subjects holds references to the objects the role applies to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


