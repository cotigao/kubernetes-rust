# IoK8sKubernetesPkgApisExtensionsV1beta1DaemonSetSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_ready_seconds** | **i32** | The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready). | [optional] 
**revision_history_limit** | **i32** | The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10. | [optional] 
**selector** | [***::models::IoK8sApimachineryPkgApisMetaV1LabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md) |  | [optional] 
**template** | [***::models::IoK8sKubernetesPkgApiV1PodTemplateSpec**](io.k8s.kubernetes.pkg.api.v1.PodTemplateSpec.md) |  | 
**template_generation** | **i64** | DEPRECATED. A sequence number representing a specific generation of the template. Populated by the system. It can be set only during the creation. | [optional] 
**update_strategy** | [***::models::IoK8sKubernetesPkgApisExtensionsV1beta1DaemonSetUpdateStrategy**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.DaemonSetUpdateStrategy.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


