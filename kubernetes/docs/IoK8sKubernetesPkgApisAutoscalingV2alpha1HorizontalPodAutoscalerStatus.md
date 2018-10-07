# IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | [**Vec<::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerCondition>**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.HorizontalPodAutoscalerCondition.md) | conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met. | 
**current_metrics** | [**Vec<::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricStatus>**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.MetricStatus.md) | currentMetrics is the last read state of the metrics used by this autoscaler. | 
**current_replicas** | **i32** | currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler. | 
**desired_replicas** | **i32** | desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler. | 
**last_scale_time** | **String** |  | [optional] 
**observed_generation** | **i64** | observedGeneration is the most recent generation observed by this autoscaler. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


