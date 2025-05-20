# ScalewayPeriodRdbPeriodV1PeriodSnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the snapshot. | [optional]
**instance_id** | Option<**String**> | UUID of the Database Instance. | [optional]
**name** | Option<**String**> | Name of the snapshot. | [optional]
**status** | Option<**String**> | Status of the snapshot. | [optional][default to Unknown]
**size** | Option<**i32**> | Size of the snapshot. (in bytes) | [optional]
**expires_at** | Option<**String**> | Expiration date (must follow the ISO 8601 format). (RFC 3339 format) | [optional]
**created_at** | Option<**String**> | Creation date (must follow the ISO 8601 format). (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Updated date (must follow the ISO 8601 format). (RFC 3339 format) | [optional]
**instance_name** | Option<**String**> | Name of the Database Instance of the snapshot. | [optional]
**node_type** | Option<**String**> | Source node type. | [optional]
**volume_type** | Option<[**models::ScalewayRdbV1SnapshotVolumeType**](scaleway_rdb_v1_Snapshot_volume_type.md)> |  | [optional]
**region** | Option<**String**> | Region of this snapshot. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


