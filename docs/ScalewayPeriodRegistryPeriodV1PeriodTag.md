# ScalewayPeriodRegistryPeriodV1PeriodTag

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique ID of the tag | [optional]
**name** | Option<**String**> | Tag name, unique for an image | [optional]
**image_id** | Option<**String**> | Image ID this tag belongs to | [optional]
**status** | Option<**String**> | Tag status | [optional][default to Unknown]
**digest** | Option<**String**> | Hash of the tag actual content. Several tags of a same image may have the same digest | [optional]
**created_at** | Option<**String**> | Creation date (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Last modification date, from the user or the service (RFC 3339 format) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


