# CreateInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | Please use project_id instead. | [optional]
**project_id** | Option<**String**> | The Project ID on which the Database Instance will be created. | [optional]
**name** | Option<**String**> | Name of the Database Instance. | [optional]
**engine** | **String** | Database engine of the Database Instance (PostgreSQL, MySQL, ...). | 
**user_name** | **String** | Username created when the Database Instance is created. | 
**password** | **String** | Password of the user. Password must be between 8 and 128 characters, contain at least one digit, one uppercase, one lowercase and one special character. | 
**node_type** | **String** | Type of node to use for the Database Instance. | 
**is_ha_cluster** | Option<**bool**> | Defines whether or not High-Availability is enabled. | [optional]
**disable_backup** | Option<**bool**> | Defines whether or not backups are disabled. | [optional]
**tags** | Option<**Vec<String>**> | Tags to apply to the Database Instance. | [optional]
**init_settings** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodInstanceSetting>**](scaleway.rdb.v1.InstanceSetting.md)> | List of engine settings to be set upon Database Instance initialization. | [optional]
**volume_type** | Option<**String**> | Type of volume where data is stored (lssd, bssd, ...). | [optional][default to Lssd]
**volume_size** | Option<**i32**> | Volume size when volume_type is not lssd. (in bytes) | [optional]
**init_endpoints** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodEndpointSpec>**](scaleway.rdb.v1.EndpointSpec.md)> | One or multiple EndpointSpec used to expose your Database Instance. A load_balancer public endpoint is systematically created. | [optional]
**backup_same_region** | Option<**bool**> | Defines whether to or not to store logical backups in the same region as the Database Instance. | [optional]
**encryption** | Option<[**models::CreateInstanceRequestEncryption**](CreateInstance_request_encryption.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


