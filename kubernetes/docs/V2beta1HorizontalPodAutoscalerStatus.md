# V2beta1HorizontalPodAutoscalerStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | [**Vec<::models::V2beta1HorizontalPodAutoscalerCondition>**](v2beta1.HorizontalPodAutoscalerCondition.md) | conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met. | [default to null]
**current_metrics** | [**Vec<::models::V2beta1MetricStatus>**](v2beta1.MetricStatus.md) | currentMetrics is the last read state of the metrics used by this autoscaler. | [optional] [default to null]
**current_replicas** | **i32** | currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler. | [default to null]
**desired_replicas** | **i32** | desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler. | [default to null]
**last_scale_time** | **String** | lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed. | [optional] [default to null]
**observed_generation** | **i64** | observedGeneration is the most recent generation observed by this autoscaler. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


