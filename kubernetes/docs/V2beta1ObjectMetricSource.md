# V2beta1ObjectMetricSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**average_value** | **String** | averageValue is the target value of the average of the metric across all relevant pods (as a quantity) | [optional] [default to null]
**metric_name** | **String** | metricName is the name of the metric in question. | [default to null]
**selector** | [***::models::V1LabelSelector**](v1.LabelSelector.md) | selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping When unset, just the metricName will be used to gather metrics. | [optional] [default to null]
**target** | [***::models::V2beta1CrossVersionObjectReference**](v2beta1.CrossVersionObjectReference.md) | target is the described Kubernetes object. | [default to null]
**target_value** | **String** | targetValue is the target value of the metric (as a quantity). | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


