# IoK8sKubernetesPkgApiV1ServicePort

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. This maps to the &#39;Name&#39; field in EndpointPort objects. Optional if only one ServicePort is defined on this service. | [optional] 
**node_port** | **i32** | The port on each node on which this service is exposed when type&#x3D;NodePort or LoadBalancer. Usually assigned by the system. If specified, it will be allocated to the service if unused or else creation of the service will fail. Default is to auto-allocate a port if the ServiceType of this Service requires one. More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport | [optional] 
**port** | **i32** | The port that will be exposed by this service. | 
**protocol** | **String** | The IP protocol for this port. Supports \&quot;TCP\&quot; and \&quot;UDP\&quot;. Default is TCP. | [optional] 
**target_port** | **String** |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


