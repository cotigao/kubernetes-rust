# V1beta1CustomResourceColumnDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**json_path** | **String** | JSONPath is a simple JSON path, i.e. with array notation. | [default to null]
**description** | **String** | description is a human readable description of this column. | [optional] [default to null]
**format** | **String** | format is an optional OpenAPI type definition for this column. The &#39;name&#39; format is applied to the primary identifier column to assist in clients identifying column is the resource name. See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for more. | [optional] [default to null]
**name** | **String** | name is a human readable name for the column. | [default to null]
**priority** | **i32** | priority is an integer defining the relative importance of this column compared to others. Lower numbers are considered higher priority. Columns that may be omitted in limited space scenarios should be given a higher priority. | [optional] [default to null]
**_type** | **String** | type is an OpenAPI type definition for this column. See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for more. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


