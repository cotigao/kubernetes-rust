# V1beta1VolumeAttachmentSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attacher** | **String** | Attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName(). | [default to null]
**node_name** | **String** | The node that the volume should be attached to. | [default to null]
**source** | [***::models::V1beta1VolumeAttachmentSource**](v1beta1.VolumeAttachmentSource.md) | Source represents the volume that should be attached. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


