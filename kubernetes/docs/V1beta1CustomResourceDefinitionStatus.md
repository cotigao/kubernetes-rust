# V1beta1CustomResourceDefinitionStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_names** | [***::models::V1beta1CustomResourceDefinitionNames**](v1beta1.CustomResourceDefinitionNames.md) | AcceptedNames are the names that are actually being used to serve discovery They may be different than the names in spec. | [default to null]
**conditions** | [**Vec<::models::V1beta1CustomResourceDefinitionCondition>**](v1beta1.CustomResourceDefinitionCondition.md) | Conditions indicate state for particular aspects of a CustomResourceDefinition | [default to null]
**stored_versions** | **Vec<String>** | StoredVersions are all versions of CustomResources that were ever persisted. Tracking these versions allows a migration path for stored versions in etcd. The field is mutable so the migration controller can first finish a migration to another version (i.e. that no old objects are left in the storage), and then remove the rest of the versions from this list. None of the versions in this list can be removed from the spec.Versions field. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


