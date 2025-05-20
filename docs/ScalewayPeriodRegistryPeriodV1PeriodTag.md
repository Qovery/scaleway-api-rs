# ScalewayPeriodRegistryPeriodV1PeriodTag

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the tag. | [optional]
**name** | Option<**String**> | Tag name, unique to an image. | [optional]
**image_id** | Option<**String**> | Image ID the of the image the tag belongs to. | [optional]
**status** | Option<**String**> | Tag status. | [optional][default to Unknown]
**digest** | Option<**String**> | Hash of the tag content. Several tags of a same image may have the same digest. | [optional]
**created_at** | Option<**String**> | Date and time of creation. (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Date and time of last update. (RFC 3339 format) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


