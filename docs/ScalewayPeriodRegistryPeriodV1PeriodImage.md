# ScalewayPeriodRegistryPeriodV1PeriodImage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique ID of the Image | [optional]
**name** | Option<**String**> | The Image name, unique in a namespace | [optional]
**namespace_id** | Option<**String**> | The unique ID of the Namespace the image belongs to | [optional]
**status** | Option<**String**> | The status of the image | [optional][default to Unknown]
**status_message** | Option<**String**> | Details of the image status | [optional]
**visibility** | Option<**String**> | A `public` image is pullable from internet without authentication, opposed to a `private` image. `inherit` will use the namespace `is_public` parameter | [optional][default to VisibilityUnknown]
**size** | Option<**i32**> | Image size in bytes, calculated from the size of image layers. One layer used in two tags of the same image is counted once but one layer used in two images is counted twice. (in bytes) | [optional]
**created_at** | Option<**String**> | Creation date (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Last modification date, from the user or the service (RFC 3339 format) | [optional]
**tags** | Option<**Vec<String>**> | List of docker tags of the image | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


