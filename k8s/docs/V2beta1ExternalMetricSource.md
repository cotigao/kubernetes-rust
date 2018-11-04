# V2beta1ExternalMetricSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metric_name** | **String** | metricName is the name of the metric in question. | [default to null]
**metric_selector** | [***::models::V1LabelSelector**](v1.LabelSelector.md) | metricSelector is used to identify a specific time series within a given metric. | [optional] [default to null]
**target_average_value** | **String** | targetAverageValue is the target per-pod value of global metric (as a quantity). Mutually exclusive with TargetValue. | [optional] [default to null]
**target_value** | **String** | targetValue is the target value of the metric (as a quantity). Mutually exclusive with TargetAverageValue. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


