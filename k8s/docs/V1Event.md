# V1Event

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | What action was taken/failed regarding to the Regarding object. | [optional] [default to null]
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] [default to null]
**count** | **i32** | The number of times this event has occurred. | [optional] [default to null]
**event_time** | **String** | Time when this Event was first observed. | [optional] [default to null]
**first_timestamp** | **String** | The time at which the event was first recorded. (Time of server receipt is in TypeMeta.) | [optional] [default to null]
**involved_object** | [***::models::V1ObjectReference**](v1.ObjectReference.md) | The object that this event is about. | [default to null]
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] [default to null]
**last_timestamp** | **String** | The time at which the most recent occurrence of this event was recorded. | [optional] [default to null]
**message** | **String** | A human-readable description of the status of this operation. | [optional] [default to null]
**metadata** | [***::models::V1ObjectMeta**](v1.ObjectMeta.md) | Standard object&#39;s metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata | [default to null]
**reason** | **String** | This should be a short, machine understandable string that gives the reason for the transition into the object&#39;s current status. | [optional] [default to null]
**related** | [***::models::V1ObjectReference**](v1.ObjectReference.md) | Optional secondary object for more complex actions. | [optional] [default to null]
**reporting_component** | **String** | Name of the controller that emitted this Event, e.g. &#x60;kubernetes.io/kubelet&#x60;. | [optional] [default to null]
**reporting_instance** | **String** | ID of the controller instance, e.g. &#x60;kubelet-xyzf&#x60;. | [optional] [default to null]
**series** | [***::models::V1EventSeries**](v1.EventSeries.md) | Data about the Event series this event represents or nil if it&#39;s a singleton Event. | [optional] [default to null]
**source** | [***::models::V1EventSource**](v1.EventSource.md) | The component reporting this event. Should be a short machine understandable string. | [optional] [default to null]
**_type** | **String** | Type of this event (Normal, Warning), new types could be added in the future | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


