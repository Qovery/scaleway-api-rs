# ScalewayPeriodRdbPeriodV1PeriodEngineSetting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Setting name from the database engine. | [optional]
**default_value** | Option<**String**> | Value set when not specified. | [optional]
**hot_configurable** | Option<**bool**> | Setting can be applied without restarting. | [optional]
**description** | Option<**String**> | Setting description. | [optional]
**property_type** | Option<**String**> | Setting type. | [optional][default to Boolean]
**unit** | Option<**String**> | Setting base unit. | [optional]
**string_constraint** | Option<**String**> | Validation regex for string type settings. | [optional]
**int_min** | Option<**i32**> | Minimum value for int types. | [optional]
**int_max** | Option<**i32**> | Maximum value for int types. | [optional]
**float_min** | Option<[**models::ScalewayRdbV1EngineSettingFloatMin**](scaleway_rdb_v1_EngineSetting_float_min.md)> |  | [optional]
**float_max** | Option<[**models::ScalewayRdbV1EngineSettingFloatMax**](scaleway_rdb_v1_EngineSetting_float_max.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


