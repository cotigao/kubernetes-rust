# IoK8sKubernetesPkgApisSettingsV1alpha1PodPresetSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**env** | [**Vec<::models::IoK8sKubernetesPkgApiV1EnvVar>**](io.k8s.kubernetes.pkg.api.v1.EnvVar.md) | Env defines the collection of EnvVar to inject into containers. | [optional] [default to null]
**env_from** | [**Vec<::models::IoK8sKubernetesPkgApiV1EnvFromSource>**](io.k8s.kubernetes.pkg.api.v1.EnvFromSource.md) | EnvFrom defines the collection of EnvFromSource to inject into containers. | [optional] [default to null]
**selector** | [***::models::IoK8sApimachineryPkgApisMetaV1LabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md) | Selector is a label query over a set of resources, in this case pods. Required. | [optional] [default to null]
**volume_mounts** | [**Vec<::models::IoK8sKubernetesPkgApiV1VolumeMount>**](io.k8s.kubernetes.pkg.api.v1.VolumeMount.md) | VolumeMounts defines the collection of VolumeMount to inject into containers. | [optional] [default to null]
**volumes** | [**Vec<::models::IoK8sKubernetesPkgApiV1Volume>**](io.k8s.kubernetes.pkg.api.v1.Volume.md) | Volumes defines the collection of Volume to inject into the pod. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


