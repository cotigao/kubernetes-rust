# V1EnvFromSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_map_ref** | [***::models::V1ConfigMapEnvSource**](v1.ConfigMapEnvSource.md) | The ConfigMap to select from | [optional] [default to null]
**prefix** | **String** | An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER. | [optional] [default to null]
**secret_ref** | [***::models::V1SecretEnvSource**](v1.SecretEnvSource.md) | The Secret to select from | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


