# ScalewayRdbV1Instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Creation date (Format ISO 8601) | [optional]
**volume** | Option<[**crate::models::ScalewayRdbV1InstanceVolume**](scaleway_rdb_v1_Instance_volume.md)> |  | [optional]
**region** | Option<**String**> | Region the instance is in | [optional]
**id** | Option<**String**> | UUID of the instance | [optional]
**name** | Option<**String**> | Name of the instance | [optional]
**organization_id** | Option<**String**> | Organization ID the instance belongs to | [optional]
**project_id** | Option<**String**> | Project ID the instance belongs to | [optional]
**status** | Option<**String**> | Status of the instance | [optional][default to Status_Unknown]
**engine** | Option<**String**> | Database engine of the database (PostgreSQL, MySQL, ...) | [optional]
**endpoint** | Option<[**crate::models::ScalewayRdbV1InstanceEndpoint**](scaleway_rdb_v1_Instance_endpoint.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | List of tags applied to the instance | [optional]
**settings** | Option<[**Vec<crate::models::ScalewayRdbV1InstanceSetting>**](scaleway.rdb.v1.InstanceSetting.md)> | Advanced settings of the instance | [optional]
**backup_schedule** | Option<[**crate::models::ScalewayRdbV1InstanceBackupSchedule**](scaleway_rdb_v1_Instance_backup_schedule.md)> |  | [optional]
**is_ha_cluster** | Option<**bool**> | Whether or not High-Availability is enabled | [optional]
**read_replicas** | Option<[**Vec<crate::models::ScalewayRdbV1Endpoint>**](scaleway.rdb.v1.Endpoint.md)> | Read replicas of the instance | [optional]
**node_type** | Option<**String**> | Node type of the instance | [optional]
**init_settings** | Option<[**Vec<crate::models::ScalewayRdbV1InstanceSetting>**](scaleway.rdb.v1.InstanceSetting.md)> | List of engine settings to be set at database initialisation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


