# ScalewayPeriodRdbPeriodV1PeriodDatabaseBackup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the database backup | [optional]
**instance_id** | Option<**String**> | UUID of the instance | [optional]
**database_name** | Option<**String**> | Name of the database of this backup | [optional]
**name** | Option<**String**> | Name of the backup | [optional]
**status** | Option<**String**> | Status of the backup | [optional][default to Unknown]
**size** | Option<**i32**> | Size of the database backup (in bytes) | [optional]
**expires_at** | Option<**String**> | Expiration date (Format ISO 8601) (RFC 3339 format) | [optional]
**created_at** | Option<**String**> | Creation date (Format ISO 8601) (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Updated date (Format ISO 8601) (RFC 3339 format) | [optional]
**instance_name** | Option<**String**> | Name of the instance of the backup | [optional]
**download_url** | Option<**String**> | URL you can download the backup from | [optional]
**download_url_expires_at** | Option<**String**> | Expiration date of the download link (RFC 3339 format) | [optional]
**region** | Option<**String**> | Region of this database backup | [optional]
**same_region** | Option<**bool**> | Store logical backups in the same region as the source database instance | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


