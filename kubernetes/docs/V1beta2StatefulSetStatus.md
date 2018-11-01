# V1beta2StatefulSetStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collision_count** | **i32** | collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision. | [optional] [default to null]
**conditions** | [**Vec<::models::V1beta2StatefulSetCondition>**](v1beta2.StatefulSetCondition.md) | Represents the latest available observations of a statefulset&#39;s current state. | [optional] [default to null]
**current_replicas** | **i32** | currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision. | [optional] [default to null]
**current_revision** | **String** | currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [0,currentReplicas). | [optional] [default to null]
**observed_generation** | **i64** | observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet&#39;s generation, which is updated on mutation by the API Server. | [optional] [default to null]
**ready_replicas** | **i32** | readyReplicas is the number of Pods created by the StatefulSet controller that have a Ready Condition. | [optional] [default to null]
**replicas** | **i32** | replicas is the number of Pods created by the StatefulSet controller. | [default to null]
**update_revision** | **String** | updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [replicas-updatedReplicas,replicas) | [optional] [default to null]
**updated_replicas** | **i32** | updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


