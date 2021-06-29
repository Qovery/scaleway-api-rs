# InlineObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the image | [optional]
**root_volume** | **String** | UUID of the snapshot | 
**arch** | **String** | Architecture of the image | [default to Arch_X8664]
**default_bootscript** | Option<**String**> | Default bootscript of the image | [optional]
**extra_volumes** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Additional volumes of the image | [optional]
**organization** | Option<**String**> | Organization ID of the image | [optional]
**project** | Option<**String**> | Project ID of the image | [optional]
**public** | Option<**bool**> | True to create a public image | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


