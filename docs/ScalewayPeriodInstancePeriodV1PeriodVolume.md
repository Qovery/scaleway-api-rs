# ScalewayPeriodInstancePeriodV1PeriodVolume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The volume unique ID | [optional]
**name** | Option<**String**> | The volume name | [optional]
**export_uri** | Option<**String**> | Show the volume NBD export URI | [optional]
**size** | Option<**i32**> | The volume disk size (in bytes) | [optional]
**volume_type** | Option<**String**> | The volume type | [optional][default to LSsd]
**creation_date** | Option<**String**> | The volume creation date (RFC 3339 format) | [optional]
**modification_date** | Option<**String**> | The volume modification date (RFC 3339 format) | [optional]
**organization** | Option<**String**> | The volume organization ID | [optional]
**project** | Option<**String**> | The volume project ID | [optional]
**tags** | Option<**Vec<String>**> | The volume tags | [optional]
**server** | Option<[**crate::models::SetVolumeRequestServer**](SetVolume_request_server.md)> |  | [optional]
**state** | Option<**String**> | The volume state | [optional][default to Available]
**zone** | Option<**String**> | The zone in which is the volume | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


