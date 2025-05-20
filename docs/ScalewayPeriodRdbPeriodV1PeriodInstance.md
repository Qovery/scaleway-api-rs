# ScalewayPeriodRdbPeriodV1PeriodInstance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Creation date (must follow the ISO 8601 format). (RFC 3339 format) | [optional]
**volume** | Option<[**models::ScalewayRdbV1InstanceVolume**](scaleway_rdb_v1_Instance_volume.md)> |  | [optional]
**region** | Option<**String**> | Region the Database Instance is in. | [optional]
**id** | Option<**String**> | UUID of the Database Instance. (UUID format) | [optional]
**name** | Option<**String**> | Name of the Database Instance. | [optional]
**organization_id** | Option<**String**> | Organization ID the Database Instance belongs to. (UUID format) | [optional]
**project_id** | Option<**String**> | Project ID the Database Instance belongs to. (UUID format) | [optional]
**status** | Option<**String**> | Status of the Database Instance. | [optional][default to Unknown]
**engine** | Option<**String**> | Database engine of the database (PostgreSQL, MySQL, ...). | [optional]
**upgradable_version** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodUpgradableVersion>**](scaleway.rdb.v1.UpgradableVersion.md)> | Available database engine versions for upgrade. | [optional]
**endpoint** | Option<[**models::ScalewayRdbV1InstanceEndpoint**](scaleway_rdb_v1_Instance_endpoint.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | List of tags applied to the Database Instance. | [optional]
**settings** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodInstanceSetting>**](scaleway.rdb.v1.InstanceSetting.md)> | Advanced settings of the Database Instance. | [optional]
**backup_schedule** | Option<[**models::ScalewayRdbV1InstanceBackupSchedule**](scaleway_rdb_v1_Instance_backup_schedule.md)> |  | [optional]
**is_ha_cluster** | Option<**bool**> | Defines whether or not High-Availability is enabled. | [optional]
**read_replicas** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodReadReplica>**](scaleway.rdb.v1.ReadReplica.md)> | Read Replicas of the Database Instance. | [optional]
**node_type** | Option<**String**> | Node type of the Database Instance. | [optional]
**init_settings** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodInstanceSetting>**](scaleway.rdb.v1.InstanceSetting.md)> | List of engine settings to be set at database initialization. | [optional]
**endpoints** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodEndpoint>**](scaleway.rdb.v1.Endpoint.md)> | List of Database Instance endpoints. | [optional]
**logs_policy** | Option<[**models::UpdateInstanceRequestLogsPolicy**](UpdateInstance_request_logs_policy.md)> |  | [optional]
**backup_same_region** | Option<**bool**> | Store logical backups in the same region as the Database Instance. | [optional]
**maintenances** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodMaintenance>**](scaleway.rdb.v1.Maintenance.md)> | List of Database Instance maintenance events. | [optional]
**encryption** | Option<[**models::CreateInstanceRequestEncryption**](CreateInstance_request_encryption.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


