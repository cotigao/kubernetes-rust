# IoK8sKubernetesPkgApisAutoscalingV1HorizontalPodAutoscalerSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_replicas** | **i32** | upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas. | 
**min_replicas** | **i32** | lower limit for the number of pods that can be set by the autoscaler, default 1. | [optional] 
**scale_target_ref** | [***::models::IoK8sKubernetesPkgApisAutoscalingV1CrossVersionObjectReference**](io.k8s.kubernetes.pkg.apis.autoscaling.v1.CrossVersionObjectReference.md) |  | 
**target_cpu_utilization_percentage** | **i32** | target average CPU utilization (represented as a percentage of requested CPU) over all the pods; if not specified the default autoscaling policy will be used. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


