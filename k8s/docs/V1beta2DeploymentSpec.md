# V1beta2DeploymentSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_ready_seconds** | **i32** | Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready) | [optional] [default to null]
**paused** | **bool** | Indicates that the deployment is paused. | [optional] [default to null]
**progress_deadline_seconds** | **i32** | The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. Defaults to 600s. | [optional] [default to null]
**replicas** | **i32** | Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1. | [optional] [default to null]
**revision_history_limit** | **i32** | The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10. | [optional] [default to null]
**selector** | [***::models::V1LabelSelector**](v1.LabelSelector.md) | Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment. It must match the pod template&#39;s labels. | [default to null]
**strategy** | [***::models::V1beta2DeploymentStrategy**](v1beta2.DeploymentStrategy.md) | The deployment strategy to use to replace existing pods with new ones. | [optional] [default to null]
**template** | [***::models::V1PodTemplateSpec**](v1.PodTemplateSpec.md) | Template describes the pods that will be created. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


