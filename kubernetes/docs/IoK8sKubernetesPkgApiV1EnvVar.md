# IoK8sKubernetesPkgApiV1EnvVar

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the environment variable. Must be a C_IDENTIFIER. | [default to null]
**value** | **String** | Variable references $(VAR_NAME) are expanded using the previous defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to \&quot;\&quot;. | [optional] [default to null]
**value_from** | [***::models::IoK8sKubernetesPkgApiV1EnvVarSource**](io.k8s.kubernetes.pkg.api.v1.EnvVarSource.md) | Source for the environment variable&#39;s value. Cannot be used if value is not empty. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

