# IoK8sKubernetesPkgApisAuthorizationV1beta1SubjectAccessReviewSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extra** | [**::std::collections::HashMap<String, Vec<String>>**](array.md) | Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here. | [optional] 
**group** | **Vec<String>** | Groups is the groups you&#39;re testing for. | [optional] 
**non_resource_attributes** | [***::models::IoK8sKubernetesPkgApisAuthorizationV1beta1NonResourceAttributes**](io.k8s.kubernetes.pkg.apis.authorization.v1beta1.NonResourceAttributes.md) |  | [optional] 
**resource_attributes** | [***::models::IoK8sKubernetesPkgApisAuthorizationV1beta1ResourceAttributes**](io.k8s.kubernetes.pkg.apis.authorization.v1beta1.ResourceAttributes.md) |  | [optional] 
**user** | **String** | User is the user you&#39;re testing for. If you specify \&quot;User\&quot; but not \&quot;Group\&quot;, then is it interpreted as \&quot;What if User were not a member of any groups | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


