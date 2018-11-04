# V1AzureFilePersistentVolumeSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**read_only** | **bool** | Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. | [optional] [default to null]
**secret_name** | **String** | the name of secret that contains Azure Storage Account Name and Key | [default to null]
**secret_namespace** | **String** | the namespace of the secret that contains Azure Storage Account Name and Key default is the same as the Pod | [optional] [default to null]
**share_name** | **String** | Share Name | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


