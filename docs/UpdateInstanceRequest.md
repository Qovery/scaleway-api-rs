# UpdateInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_schedule_frequency** | Option<**i32**> | In hours. | [optional]
**backup_schedule_retention** | Option<**i32**> | In days. | [optional]
**is_backup_schedule_disabled** | Option<**bool**> | Defines whether or not the backup schedule is disabled. | [optional]
**name** | Option<**String**> | Name of the Database Instance. | [optional]
**tags** | Option<**Vec<String>**> | Tags of a Database Instance. | [optional]
**logs_policy** | Option<[**models::UpdateInstanceRequestLogsPolicy**](UpdateInstance_request_logs_policy.md)> |  | [optional]
**backup_same_region** | Option<**bool**> | Store logical backups in the same region as the Database Instance. | [optional]
**backup_schedule_start_hour** | Option<**i32**> | Defines the start time of the autobackup. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


