# IoK8sKubernetesPkgApiV1EndpointSubset

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | [**Vec<::models::IoK8sKubernetesPkgApiV1EndpointAddress>**](io.k8s.kubernetes.pkg.api.v1.EndpointAddress.md) | IP addresses which offer the related ports that are marked as ready. These endpoints should be considered safe for load balancers and clients to utilize. | [optional] [default to null]
**not_ready_addresses** | [**Vec<::models::IoK8sKubernetesPkgApiV1EndpointAddress>**](io.k8s.kubernetes.pkg.api.v1.EndpointAddress.md) | IP addresses which offer the related ports but are not currently marked as ready because they have not yet finished starting, have recently failed a readiness check, or have recently failed a liveness check. | [optional] [default to null]
**ports** | [**Vec<::models::IoK8sKubernetesPkgApiV1EndpointPort>**](io.k8s.kubernetes.pkg.api.v1.EndpointPort.md) | Port numbers available on the related IP addresses. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


