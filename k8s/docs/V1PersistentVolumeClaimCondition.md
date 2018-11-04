# V1PersistentVolumeClaimCondition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_probe_time** | **String** | Last time we probed the condition. | [optional] [default to null]
**last_transition_time** | **String** | Last time the condition transitioned from one status to another. | [optional] [default to null]
**message** | **String** | Human-readable message indicating details about last transition. | [optional] [default to null]
**reason** | **String** | Unique, this should be a short, machine understandable string that gives the reason for condition&#39;s last transition. If it reports \&quot;ResizeStarted\&quot; that means the underlying persistent volume is being resized. | [optional] [default to null]
**status** | **String** |  | [default to null]
**_type** | **String** |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


