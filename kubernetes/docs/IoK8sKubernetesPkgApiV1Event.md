# IoK8sKubernetesPkgApiV1Event

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] 
**count** | **i32** | The number of times this event has occurred. | [optional] 
**first_timestamp** | **String** |  | [optional] 
**involved_object** | [***::models::IoK8sKubernetesPkgApiV1ObjectReference**](io.k8s.kubernetes.pkg.api.v1.ObjectReference.md) |  | 
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] 
**last_timestamp** | **String** |  | [optional] 
**message** | **String** | A human-readable description of the status of this operation. | [optional] 
**metadata** | [***::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta**](io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.md) |  | 
**reason** | **String** | This should be a short, machine understandable string that gives the reason for the transition into the object&#39;s current status. | [optional] 
**source** | [***::models::IoK8sKubernetesPkgApiV1EventSource**](io.k8s.kubernetes.pkg.api.v1.EventSource.md) |  | [optional] 
**_type** | **String** | Type of this event (Normal, Warning), new types could be added in the future | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


