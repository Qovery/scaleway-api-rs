# ScalewayPeriodRegistryPeriodV1PeriodNamespace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the namespace. | [optional]
**name** | Option<**String**> | Name of the namespace, unique in a region across all organizations. | [optional]
**description** | Option<**String**> | Description of the namespace. | [optional]
**organization_id** | Option<**String**> | Owner of the namespace. | [optional]
**project_id** | Option<**String**> | Project of the namespace. | [optional]
**status** | Option<**String**> | Namespace status. | [optional][default to Unknown]
**status_message** | Option<**String**> | Namespace status details. | [optional]
**endpoint** | Option<**String**> | Endpoint reachable by docker. | [optional]
**is_public** | Option<**bool**> | Defines whether or not namespace is public. | [optional]
**size** | Option<**i32**> | Total size of the namespace, calculated as the sum of the size of all images in the namespace. (in bytes) | [optional]
**created_at** | Option<**String**> | Date and time of creation. (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Date and time of last update. (RFC 3339 format) | [optional]
**image_count** | Option<**i32**> | Number of images in the namespace. | [optional]
**region** | Option<**String**> | Region the namespace belongs to. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


