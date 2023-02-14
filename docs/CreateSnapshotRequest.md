# CreateSnapshotRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the snapshot | [optional]
**volume_id** | Option<**String**> | UUID of the volume | [optional]
**tags** | Option<**Vec<String>**> | The tags of the snapshot | [optional]
**organization** | Option<**String**> | Organization ID of the snapshot | [optional]
**project** | Option<**String**> | Project ID of the snapshot | [optional]
**volume_type** | Option<**String**> | Overrides the volume_type of the snapshot. If omitted, the volume type of the original volume will be used.  | [optional][default to UnknownVolumeType]
**bucket** | Option<**String**> | Bucket name for snapshot imports | [optional]
**key** | Option<**String**> | Object key for snapshot imports | [optional]
**size** | Option<**i32**> | Imported snapshot size, must be a multiple of 512 (in bytes) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


