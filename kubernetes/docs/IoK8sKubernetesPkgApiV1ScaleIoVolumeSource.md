# IoK8sKubernetesPkgApiV1ScaleIoVolumeSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fs_type** | **String** | Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \&quot;ext4\&quot;, \&quot;xfs\&quot;, \&quot;ntfs\&quot;. Implicitly inferred to be \&quot;ext4\&quot; if unspecified. | [optional] 
**gateway** | **String** | The host address of the ScaleIO API Gateway. | 
**protection_domain** | **String** | The name of the Protection Domain for the configured storage (defaults to \&quot;default\&quot;). | [optional] 
**read_only** | **bool** | Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. | [optional] 
**secret_ref** | [***::models::IoK8sKubernetesPkgApiV1LocalObjectReference**](io.k8s.kubernetes.pkg.api.v1.LocalObjectReference.md) |  | 
**ssl_enabled** | **bool** | Flag to enable/disable SSL communication with Gateway, default false | [optional] 
**storage_mode** | **String** | Indicates whether the storage for a volume should be thick or thin (defaults to \&quot;thin\&quot;). | [optional] 
**storage_pool** | **String** | The Storage Pool associated with the protection domain (defaults to \&quot;default\&quot;). | [optional] 
**system** | **String** | The name of the storage system as configured in ScaleIO. | 
**volume_name** | **String** | The name of a volume already created in the ScaleIO system that is associated with this volume source. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


