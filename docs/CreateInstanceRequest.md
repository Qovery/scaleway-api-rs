# CreateInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | Please use `project_id` instead | [optional]
**project_id** | Option<**String**> | The project ID on which to create the instance | [optional]
**name** | Option<**String**> | Name of the instance | [optional]
**engine** | **String** | Database engine of the database (PostgreSQL, MySQL, ...) | 
**user_name** | **String** | Name of the user created when the instance is created | 
**password** | **String** | Password of the user | 
**node_type** | **String** | Type of node to use for the instance | 
**is_ha_cluster** | Option<**bool**> | Whether or not High-Availability is enabled | [optional]
**disable_backup** | Option<**bool**> | Whether or not backups are disabled | [optional]
**tags** | Option<**Vec<String>**> | Tags to apply to the instance | [optional]
**init_settings** | Option<[**Vec<crate::models::ScalewayRdbV1InstanceSetting>**](scaleway.rdb.v1.InstanceSetting.md)> | List of engine settings to be set at database initialisation | [optional]
**volume_type** | Option<**String**> | Type of volume where data are stored (lssd, bssd, ...) | [optional][default to VolumeType_Lssd]
**volume_size** | Option<**i64**> | Volume size when volume_type is not lssd (in bytes) | [optional]
**init_endpoints** | Option<[**Vec<crate::models::ScalewayRdbV1EndpointSpec>**](scaleway.rdb.v1.EndpointSpec.md)> | One or multiple EndpointSpec used to expose your database instance | [optional]
**backup_same_region** | Option<**bool**> | Store logical backups in the same region as the database instance | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


