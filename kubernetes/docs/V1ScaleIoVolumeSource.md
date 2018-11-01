# V1ScaleIoVolumeSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fs_type** | **String** | Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \&quot;ext4\&quot;, \&quot;xfs\&quot;, \&quot;ntfs\&quot;. Default is \&quot;xfs\&quot;. | [optional] [default to null]
**gateway** | **String** | The host address of the ScaleIO API Gateway. | [default to null]
**protection_domain** | **String** | The name of the ScaleIO Protection Domain for the configured storage. | [optional] [default to null]
**read_only** | **bool** | Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. | [optional] [default to null]
**secret_ref** | [***::models::V1LocalObjectReference**](v1.LocalObjectReference.md) | SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail. | [default to null]
**ssl_enabled** | **bool** | Flag to enable/disable SSL communication with Gateway, default false | [optional] [default to null]
**storage_mode** | **String** | Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned. | [optional] [default to null]
**storage_pool** | **String** | The ScaleIO Storage Pool associated with the protection domain. | [optional] [default to null]
**system** | **String** | The name of the storage system as configured in ScaleIO. | [default to null]
**volume_name** | **String** | The name of a volume already created in the ScaleIO system that is associated with this volume source. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


