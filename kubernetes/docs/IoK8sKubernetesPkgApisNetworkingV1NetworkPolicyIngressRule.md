# IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyIngressRule

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from** | [**Vec<::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyPeer>**](io.k8s.kubernetes.pkg.apis.networking.v1.NetworkPolicyPeer.md) | List of sources which should be able to access the pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all sources (traffic not restricted by source). If this field is present and contains at least on item, this rule allows traffic only if the traffic matches at least one item in the from list. | [optional] [default to null]
**ports** | [**Vec<::models::IoK8sKubernetesPkgApisNetworkingV1NetworkPolicyPort>**](io.k8s.kubernetes.pkg.apis.networking.v1.NetworkPolicyPort.md) | List of ports which should be made accessible on the pods selected for this rule. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

