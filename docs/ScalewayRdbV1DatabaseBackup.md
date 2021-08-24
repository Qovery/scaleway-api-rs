# ScalewayRdbV1DatabaseBackup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the database backup | [optional]
**instance_id** | Option<**String**> | UUID of the instance | [optional]
**database_name** | Option<**String**> | Name of the database of this backup | [optional]
**name** | Option<**String**> | Name of the backup | [optional]
**status** | Option<**String**> | Status of the backup | [optional][default to Status_Unknown]
**size** | Option<**f32**> | Size of the database backup (in bytes) | [optional]
**expires_at** | Option<**String**> | Expiration date (Format ISO 8601) | [optional]
**created_at** | Option<**String**> | Creation date (Format ISO 8601) | [optional]
**updated_at** | Option<**String**> | Updated date (Format ISO 8601) | [optional]
**instance_name** | Option<**String**> | Name of the instance of the backup | [optional]
**download_url** | Option<**String**> | URL you can download the backup from | [optional]
**download_url_expires_at** | Option<**String**> | Expiration date of the download link | [optional]
**region** | Option<**String**> | Region of this database backup | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


