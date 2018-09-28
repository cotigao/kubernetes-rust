# IoK8sKubernetesPkgApisExtensionsV1beta1FsGroupStrategyOptions

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ranges** | [**Vec<::models::IoK8sKubernetesPkgApisExtensionsV1beta1IdRange>**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.IDRange.md) | Ranges are the allowed ranges of fs groups.  If you would like to force a single fs group then supply a single range with the same start and end. | [optional] [default to null]
**rule** | **String** | Rule is the strategy that will dictate what FSGroup is used in the SecurityContext. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


