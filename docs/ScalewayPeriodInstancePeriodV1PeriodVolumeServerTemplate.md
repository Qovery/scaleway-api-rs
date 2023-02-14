# ScalewayPeriodInstancePeriodV1PeriodVolumeServerTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the volume | [optional]
**boot** | Option<**bool**> | Force the server to boot on this volume | [optional][default to false]
**name** | Option<**String**> | Name of the volume | [optional]
**size** | Option<**i32**> | Disk size of the volume, must be a multiple of 512 (in bytes) | [optional]
**volume_type** | Option<**String**> | Type of the volume | [optional][default to LSsd]
**base_snapshot** | Option<**String**> | The ID of the snapshot on which this volume will be based | [optional]
**organization** | Option<**String**> | Organization ID of the volume | [optional]
**project** | Option<**String**> | Project ID of the volume | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


