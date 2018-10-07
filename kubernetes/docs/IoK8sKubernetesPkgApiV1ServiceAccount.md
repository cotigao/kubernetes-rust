# IoK8sKubernetesPkgApiV1ServiceAccount

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] 
**automount_service_account_token** | **bool** | AutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted. Can be overridden at the pod level. | [optional] 
**image_pull_secrets** | [**Vec<::models::IoK8sKubernetesPkgApiV1LocalObjectReference>**](io.k8s.kubernetes.pkg.api.v1.LocalObjectReference.md) | ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images in pods that reference this ServiceAccount. ImagePullSecrets are distinct from Secrets because Secrets can be mounted in the pod, but ImagePullSecrets are only accessed by the kubelet. More info: https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod | [optional] 
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] 
**metadata** | [***::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta**](io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.md) |  | [optional] 
**secrets** | [**Vec<::models::IoK8sKubernetesPkgApiV1ObjectReference>**](io.k8s.kubernetes.pkg.api.v1.ObjectReference.md) | Secrets is the list of secrets allowed to be used by pods running using this ServiceAccount. More info: https://kubernetes.io/docs/concepts/configuration/secret | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


