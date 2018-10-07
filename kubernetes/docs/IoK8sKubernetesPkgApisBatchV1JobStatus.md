# IoK8sKubernetesPkgApisBatchV1JobStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **i32** | The number of actively running pods. | [optional] 
**completion_time** | **String** |  | [optional] 
**conditions** | [**Vec<::models::IoK8sKubernetesPkgApisBatchV1JobCondition>**](io.k8s.kubernetes.pkg.apis.batch.v1.JobCondition.md) | The latest available observations of an object&#39;s current state. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/ | [optional] 
**failed** | **i32** | The number of pods which reached phase Failed. | [optional] 
**start_time** | **String** |  | [optional] 
**succeeded** | **i32** | The number of pods which reached phase Succeeded. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


