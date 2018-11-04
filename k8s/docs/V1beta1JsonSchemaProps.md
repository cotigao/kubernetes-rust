# V1beta1JsonSchemaProps

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_ref** | **String** |  | [optional] [default to null]
**schema** | **String** |  | [optional] [default to null]
**additional_items** | [***Value**](Value.md) | JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value. Defaults to true for the boolean property. | [optional] [default to null]
**additional_properties** | [***Value**](Value.md) | JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value. Defaults to true for the boolean property. | [optional] [default to null]
**all_of** | [**Vec<::models::V1beta1JsonSchemaProps>**](v1beta1.JSONSchemaProps.md) |  | [optional] [default to null]
**any_of** | [**Vec<::models::V1beta1JsonSchemaProps>**](v1beta1.JSONSchemaProps.md) |  | [optional] [default to null]
**default** | [***Value**](Value.md) | JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil. | [optional] [default to null]
**definitions** | [**::std::collections::HashMap<String, ::models::V1beta1JsonSchemaProps>**](v1beta1.JSONSchemaProps.md) |  | [optional] [default to null]
**dependencies** | [**::std::collections::HashMap<String, Value>**](Value.md) |  | [optional] [default to null]
**description** | **String** |  | [optional] [default to null]
**_enum** | [**Vec<Value>**](Value.md) |  | [optional] [default to null]
**example** | [***Value**](Value.md) | JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil. | [optional] [default to null]
**exclusive_maximum** | **bool** |  | [optional] [default to null]
**exclusive_minimum** | **bool** |  | [optional] [default to null]
**external_docs** | [***::models::V1beta1ExternalDocumentation**](v1beta1.ExternalDocumentation.md) |  | [optional] [default to null]
**format** | **String** |  | [optional] [default to null]
**id** | **String** |  | [optional] [default to null]
**items** | [***Value**](Value.md) | JSONSchemaPropsOrArray represents a value that can either be a JSONSchemaProps or an array of JSONSchemaProps. Mainly here for serialization purposes. | [optional] [default to null]
**max_items** | **i64** |  | [optional] [default to null]
**max_length** | **i64** |  | [optional] [default to null]
**max_properties** | **i64** |  | [optional] [default to null]
**maximum** | **f64** |  | [optional] [default to null]
**min_items** | **i64** |  | [optional] [default to null]
**min_length** | **i64** |  | [optional] [default to null]
**min_properties** | **i64** |  | [optional] [default to null]
**minimum** | **f64** |  | [optional] [default to null]
**multiple_of** | **f64** |  | [optional] [default to null]
**not** | [***::models::V1beta1JsonSchemaProps**](v1beta1.JSONSchemaProps.md) |  | [optional] [default to null]
**one_of** | [**Vec<::models::V1beta1JsonSchemaProps>**](v1beta1.JSONSchemaProps.md) |  | [optional] [default to null]
**pattern** | **String** |  | [optional] [default to null]
**pattern_properties** | [**::std::collections::HashMap<String, ::models::V1beta1JsonSchemaProps>**](v1beta1.JSONSchemaProps.md) |  | [optional] [default to null]
**properties** | [**::std::collections::HashMap<String, ::models::V1beta1JsonSchemaProps>**](v1beta1.JSONSchemaProps.md) |  | [optional] [default to null]
**required** | **Vec<String>** |  | [optional] [default to null]
**title** | **String** |  | [optional] [default to null]
**_type** | **String** |  | [optional] [default to null]
**unique_items** | **bool** |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


