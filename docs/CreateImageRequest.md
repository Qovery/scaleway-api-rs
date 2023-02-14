# CreateImageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the image | [optional]
**root_volume** | **String** | UUID of the snapshot | 
**arch** | **String** | Architecture of the image | [default to X8664]
**default_bootscript** | Option<**String**> | Default bootscript of the image | [optional]
**extra_volumes** | Option<[**crate::models::CreateImageRequestExtraVolumes**](CreateImage_request_extra_volumes.md)> |  | [optional]
**organization** | Option<**String**> | Organization ID of the image | [optional]
**project** | Option<**String**> | Project ID of the image | [optional]
**tags** | Option<**Vec<String>**> | The tags of the image | [optional]
**public** | Option<**bool**> | True to create a public image | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


