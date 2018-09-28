# IoK8sKubernetesPkgApiV1ResourceRequirements

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**limits** | [**::std::collections::HashMap<String, ::models::IoK8sApimachineryPkgApiResourceQuantity>**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/ | [optional] [default to null]
**requests** | [**::std::collections::HashMap<String, ::models::IoK8sApimachineryPkgApiResourceQuantity>**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/ | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


