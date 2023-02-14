# SetVolumeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The volumes names | [optional]
**tags** | Option<**Vec<String>**> | The tags of the volume | [optional]
**export_uri** | Option<**String**> | Show the volumes NBD export URI, this field is ignored | [optional]
**size** | Option<**i32**> | The volume's disk size, must be a multiple of 512 (in bytes) | [optional]
**volume_type** | Option<**String**> | The volume's type | [optional][default to LSsd]
**creation_date** | Option<**String**> | The volume's creation date (RFC 3339 format) | [optional]
**modification_date** | Option<**String**> | The volume's modification date (RFC 3339 format) | [optional]
**organization** | Option<**String**> | The volume's organization ID | [optional]
**project** | Option<**String**> | The volume's project ID | [optional]
**server** | Option<[**crate::models::SetVolumeRequestServer**](SetVolume_request_server.md)> |  | [optional]
**state** | Option<**String**> | The volume's state | [optional][default to Available]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


