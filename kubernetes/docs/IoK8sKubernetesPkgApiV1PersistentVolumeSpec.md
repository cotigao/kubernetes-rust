# IoK8sKubernetesPkgApiV1PersistentVolumeSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_modes** | **Vec<String>** | AccessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes | [optional] 
**aws_elastic_block_store** | [***::models::IoK8sKubernetesPkgApiV1AwsElasticBlockStoreVolumeSource**](io.k8s.kubernetes.pkg.api.v1.AWSElasticBlockStoreVolumeSource.md) |  | [optional] 
**azure_disk** | [***::models::IoK8sKubernetesPkgApiV1AzureDiskVolumeSource**](io.k8s.kubernetes.pkg.api.v1.AzureDiskVolumeSource.md) |  | [optional] 
**azure_file** | [***::models::IoK8sKubernetesPkgApiV1AzureFileVolumeSource**](io.k8s.kubernetes.pkg.api.v1.AzureFileVolumeSource.md) |  | [optional] 
**capacity** | **::std::collections::HashMap<String, String>** | A description of the persistent volume&#39;s resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity | [optional] 
**cephfs** | [***::models::IoK8sKubernetesPkgApiV1CephFsVolumeSource**](io.k8s.kubernetes.pkg.api.v1.CephFSVolumeSource.md) |  | [optional] 
**cinder** | [***::models::IoK8sKubernetesPkgApiV1CinderVolumeSource**](io.k8s.kubernetes.pkg.api.v1.CinderVolumeSource.md) |  | [optional] 
**claim_ref** | [***::models::IoK8sKubernetesPkgApiV1ObjectReference**](io.k8s.kubernetes.pkg.api.v1.ObjectReference.md) |  | [optional] 
**fc** | [***::models::IoK8sKubernetesPkgApiV1FcVolumeSource**](io.k8s.kubernetes.pkg.api.v1.FCVolumeSource.md) |  | [optional] 
**flex_volume** | [***::models::IoK8sKubernetesPkgApiV1FlexVolumeSource**](io.k8s.kubernetes.pkg.api.v1.FlexVolumeSource.md) |  | [optional] 
**flocker** | [***::models::IoK8sKubernetesPkgApiV1FlockerVolumeSource**](io.k8s.kubernetes.pkg.api.v1.FlockerVolumeSource.md) |  | [optional] 
**gce_persistent_disk** | [***::models::IoK8sKubernetesPkgApiV1GcePersistentDiskVolumeSource**](io.k8s.kubernetes.pkg.api.v1.GCEPersistentDiskVolumeSource.md) |  | [optional] 
**glusterfs** | [***::models::IoK8sKubernetesPkgApiV1GlusterfsVolumeSource**](io.k8s.kubernetes.pkg.api.v1.GlusterfsVolumeSource.md) |  | [optional] 
**host_path** | [***::models::IoK8sKubernetesPkgApiV1HostPathVolumeSource**](io.k8s.kubernetes.pkg.api.v1.HostPathVolumeSource.md) |  | [optional] 
**iscsi** | [***::models::IoK8sKubernetesPkgApiV1IscsiVolumeSource**](io.k8s.kubernetes.pkg.api.v1.ISCSIVolumeSource.md) |  | [optional] 
**local** | [***::models::IoK8sKubernetesPkgApiV1LocalVolumeSource**](io.k8s.kubernetes.pkg.api.v1.LocalVolumeSource.md) |  | [optional] 
**nfs** | [***::models::IoK8sKubernetesPkgApiV1NfsVolumeSource**](io.k8s.kubernetes.pkg.api.v1.NFSVolumeSource.md) |  | [optional] 
**persistent_volume_reclaim_policy** | **String** | What happens to a persistent volume when released from its claim. Valid options are Retain (default) and Recycle. Recycling must be supported by the volume plugin underlying this persistent volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming | [optional] 
**photon_persistent_disk** | [***::models::IoK8sKubernetesPkgApiV1PhotonPersistentDiskVolumeSource**](io.k8s.kubernetes.pkg.api.v1.PhotonPersistentDiskVolumeSource.md) |  | [optional] 
**portworx_volume** | [***::models::IoK8sKubernetesPkgApiV1PortworxVolumeSource**](io.k8s.kubernetes.pkg.api.v1.PortworxVolumeSource.md) |  | [optional] 
**quobyte** | [***::models::IoK8sKubernetesPkgApiV1QuobyteVolumeSource**](io.k8s.kubernetes.pkg.api.v1.QuobyteVolumeSource.md) |  | [optional] 
**rbd** | [***::models::IoK8sKubernetesPkgApiV1RbdVolumeSource**](io.k8s.kubernetes.pkg.api.v1.RBDVolumeSource.md) |  | [optional] 
**scale_io** | [***::models::IoK8sKubernetesPkgApiV1ScaleIoVolumeSource**](io.k8s.kubernetes.pkg.api.v1.ScaleIOVolumeSource.md) |  | [optional] 
**storage_class_name** | **String** | Name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass. | [optional] 
**storageos** | [***::models::IoK8sKubernetesPkgApiV1StorageOsPersistentVolumeSource**](io.k8s.kubernetes.pkg.api.v1.StorageOSPersistentVolumeSource.md) |  | [optional] 
**vsphere_volume** | [***::models::IoK8sKubernetesPkgApiV1VsphereVirtualDiskVolumeSource**](io.k8s.kubernetes.pkg.api.v1.VsphereVirtualDiskVolumeSource.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


