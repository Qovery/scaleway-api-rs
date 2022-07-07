# ScalewayRegistryV1Namespace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique ID of the namespace | [optional]
**name** | Option<**String**> | The name of the namespace, unique in a region accross all organizations | [optional]
**description** | Option<**String**> | Description of the namespace | [optional]
**organization_id** | Option<**String**> | Owner of the namespace | [optional]
**project_id** | Option<**String**> | Project of the namespace | [optional]
**status** | Option<**String**> | Namespace status | [optional][default to Status_Unknown]
**status_message** | Option<**String**> | Namespace status details | [optional]
**endpoint** | Option<**String**> | Endpoint reachable by docker | [optional]
**is_public** | Option<**bool**> | Namespace visibility policy | [optional]
**size** | Option<**i64**> | Total size of the namespace, calculated as the sum of the size of all images in the namespace (in bytes) | [optional]
**created_at** | Option<**String**> | Creation date | [optional]
**updated_at** | Option<**String**> | Last modification date, from the user or the service | [optional]
**image_count** | Option<**i64**> | Number of images in the namespace | [optional]
**region** | Option<**String**> | Region the namespace belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


