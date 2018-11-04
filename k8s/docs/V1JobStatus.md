# V1JobStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **i32** | The number of actively running pods. | [optional] [default to null]
**completion_time** | **String** | Represents time when the job was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC. | [optional] [default to null]
**conditions** | [**Vec<::models::V1JobCondition>**](v1.JobCondition.md) | The latest available observations of an object&#39;s current state. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/ | [optional] [default to null]
**failed** | **i32** | The number of pods which reached phase Failed. | [optional] [default to null]
**start_time** | **String** | Represents time when the job was acknowledged by the job controller. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC. | [optional] [default to null]
**succeeded** | **i32** | The number of pods which reached phase Succeeded. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


