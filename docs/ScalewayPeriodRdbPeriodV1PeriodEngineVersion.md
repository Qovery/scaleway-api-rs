# ScalewayPeriodRdbPeriodV1PeriodEngineVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | Option<**String**> | Database engine version. | [optional]
**name** | Option<**String**> | Database engine name. | [optional]
**end_of_life** | Option<**String**> | End of life date. (RFC 3339 format) | [optional]
**available_settings** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodEngineSetting>**](scaleway.rdb.v1.EngineSetting.md)> | Engine settings available to be set. | [optional]
**disabled** | Option<**bool**> | Disabled versions cannot be created. | [optional]
**beta** | Option<**bool**> | Beta status of engine version. | [optional]
**available_init_settings** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodEngineSetting>**](scaleway.rdb.v1.EngineSetting.md)> | Engine settings available to be set at database initialization. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


