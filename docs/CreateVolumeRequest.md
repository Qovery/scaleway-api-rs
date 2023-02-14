# CreateVolumeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The volume name | [optional]
**organization** | Option<**String**> | The volume organization ID | [optional]
**project** | Option<**String**> | The volume project ID | [optional]
**tags** | Option<**Vec<String>**> | The volume tags | [optional]
**volume_type** | Option<**String**> | The volume type | [optional][default to LSsd]
**size** | Option<**i32**> | The volume disk size, must be a multiple of 512 (in bytes) | [optional]
**base_volume** | Option<**String**> | The ID of the volume on which this volume will be based | [optional]
**base_snapshot** | Option<**String**> | The ID of the snapshot on which this volume will be based | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


