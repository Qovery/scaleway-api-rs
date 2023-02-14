# ScalewayPeriodInstancePeriodV1PeriodServerType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**monthly_price** | Option<**i64**> | Estimated monthly price, for a 30 days month, in Euro | [optional]
**hourly_price** | Option<**i64**> | Hourly price in Euro | [optional]
**alt_names** | Option<**Vec<String>**> | Alternative instance name if any | [optional]
**per_volume_constraint** | Option<[**crate::models::ScalewayInstanceV1ServerTypePerVolumeConstraint**](scaleway_instance_v1_ServerType_per_volume_constraint.md)> |  | [optional]
**volumes_constraint** | Option<[**crate::models::ScalewayInstanceV1ServerTypeVolumesConstraint**](scaleway_instance_v1_ServerType_volumes_constraint.md)> |  | [optional]
**ncpus** | Option<**i32**> | Number of CPU | [optional]
**gpu** | Option<**i32**> | Number of GPU | [optional]
**ram** | Option<**i32**> | Available RAM in bytes | [optional]
**arch** | Option<**String**> | CPU architecture | [optional][default to X8664]
**baremetal** | Option<**bool**> | True if it is a baremetal instance | [optional]
**network** | Option<[**crate::models::ScalewayInstanceV1ServerTypeNetwork**](scaleway_instance_v1_ServerType_network.md)> |  | [optional]
**capabilities** | Option<[**crate::models::ScalewayInstanceV1ServerTypeCapabilities**](scaleway_instance_v1_ServerType_capabilities.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


