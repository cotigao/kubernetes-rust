# V1beta1CustomResourceDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] [default to null]
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] [default to null]
**metadata** | [***::models::V1ObjectMeta**](v1.ObjectMeta.md) |  | [optional] [default to null]
**spec** | [***::models::V1beta1CustomResourceDefinitionSpec**](v1beta1.CustomResourceDefinitionSpec.md) | Spec describes how the user wants the resources to appear | [default to null]
**status** | [***::models::V1beta1CustomResourceDefinitionStatus**](v1beta1.CustomResourceDefinitionStatus.md) | Status indicates the actual state of the CustomResourceDefinition | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


