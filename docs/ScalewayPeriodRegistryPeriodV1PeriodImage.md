# ScalewayPeriodRegistryPeriodV1PeriodImage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the image. | [optional]
**name** | Option<**String**> | Name of the image, it must be unique within the namespace. | [optional]
**namespace_id** | Option<**String**> | UUID of the namespace the image belongs to. | [optional]
**status** | Option<**String**> | Status of the image. | [optional][default to Unknown]
**status_message** | Option<**String**> | Details of the image status. | [optional]
**visibility** | Option<**String**> | Set to `public` to allow the image to be pulled without authentication. Else, set to  `private`. Set to `inherit` to keep the same visibility configuration as the namespace. | [optional][default to VisibilityUnknown]
**size** | Option<**i32**> | Image size in bytes, calculated from the size of image layers. Image size in bytes, calculated from the size of image layers. One layer used in two tags of the same image is counted once but one layer used in two images is counted twice. (in bytes) | [optional]
**created_at** | Option<**String**> | Date and time of image creation. (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Date and time of last update. (RFC 3339 format) | [optional]
**tags** | Option<**Vec<String>**> | List of docker tags of the image. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


