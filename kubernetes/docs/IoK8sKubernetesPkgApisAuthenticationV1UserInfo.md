# IoK8sKubernetesPkgApisAuthenticationV1UserInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extra** | [**::std::collections::HashMap<String, Vec<String>>**](array.md) | Any additional information provided by the authenticator. | [optional] [default to null]
**groups** | **Vec<String>** | The names of groups this user is a part of. | [optional] [default to null]
**uid** | **String** | A unique value that identifies this user across time. If this user is deleted and another user by the same name is added, they will have different UIDs. | [optional] [default to null]
**username** | **String** | The name that uniquely identifies this user among all active users. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


