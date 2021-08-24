# InlineObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**offer_id** | **String** | Offer ID of the new server | 
**organization_id** | Option<**String**> | Organization ID with which the server will be created | [optional]
**project_id** | Option<**String**> | Project ID with which the server will be created | [optional]
**name** | **String** | Name of the server (≠hostname) | 
**description** | **String** | Description associated to the server, max 255 characters | 
**tags** | Option<**Vec<String>**> | Tags to associate to the server | [optional]
**install** | Option<[**crate::models::ScalewayBaremetalV1CreateServerRequestInstall**](scaleway.baremetal.v1.CreateServerRequest.Install.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


