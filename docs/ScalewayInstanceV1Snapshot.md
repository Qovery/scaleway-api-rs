# ScalewayInstanceV1Snapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The snapshot ID | [optional]
**name** | Option<**String**> | The snapshot name | [optional]
**organization** | Option<**String**> | The snapshot organization ID | [optional]
**project** | Option<**String**> | The snapshot project ID | [optional]
**volume_type** | Option<**String**> | The snapshot volume type | [optional][default to VolumeType_LSsd]
**size** | Option<**f32**> | The snapshot size (in bytes) | [optional]
**state** | Option<**String**> | The snapshot state | [optional][default to State_Available]
**base_volume** | Option<[**crate::models::ScalewayInstanceV1SnapshotBaseVolume**](scaleway_instance_v1_Snapshot_base_volume.md)> |  | [optional]
**creation_date** | Option<**String**> | The snapshot creation date | [optional]
**modification_date** | Option<**String**> | The snapshot modification date | [optional]
**zone** | Option<**String**> | The snapshot zone | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


