# IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_replicas** | **i32** | maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas. | 
**metrics** | [**Vec<::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricSpec>**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.MetricSpec.md) | metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across all metrics will be used).  The desired replica count is calculated multiplying the ratio between the target value and the current value by the current number of pods.  Ergo, metrics used must decrease as the pod count is increased, and vice-versa.  See the individual metric source types for more information about how each type of metric must respond. | [optional] 
**min_replicas** | **i32** | minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down. It defaults to 1 pod. | [optional] 
**scale_target_ref** | [***::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1CrossVersionObjectReference**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.CrossVersionObjectReference.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


