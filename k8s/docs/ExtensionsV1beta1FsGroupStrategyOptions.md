# ExtensionsV1beta1FsGroupStrategyOptions

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ranges** | [**Vec<::models::ExtensionsV1beta1IdRange>**](extensions.v1beta1.IDRange.md) | ranges are the allowed ranges of fs groups.  If you would like to force a single fs group then supply a single range with the same start and end. Required for MustRunAs. | [optional] [default to null]
**rule** | **String** | rule is the strategy that will dictate what FSGroup is used in the SecurityContext. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


