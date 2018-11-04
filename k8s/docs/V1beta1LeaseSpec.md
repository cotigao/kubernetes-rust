# V1beta1LeaseSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acquire_time** | **String** | acquireTime is a time when the current lease was acquired. | [optional] [default to null]
**holder_identity** | **String** | holderIdentity contains the identity of the holder of a current lease. | [optional] [default to null]
**lease_duration_seconds** | **i32** | leaseDurationSeconds is a duration that candidates for a lease need to wait to force acquire it. This is measure against time of last observed RenewTime. | [optional] [default to null]
**lease_transitions** | **i32** | leaseTransitions is the number of transitions of a lease between holders. | [optional] [default to null]
**renew_time** | **String** | renewTime is a time when the current holder of a lease has last updated the lease. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


