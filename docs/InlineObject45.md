# InlineObject45

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Resource name | 
**inbound_port** | **f32** | TCP port to listen on the front side | 
**backend_id** | **String** | Backend ID | 
**timeout_client** | Option<**f32**> | Set the maximum inactivity time on the client side (in milliseconds) | [optional]
**certificate_id** | Option<**String**> | Certificate ID, deprecated in favor of certificate_ids array ! | [optional]
**certificate_ids** | Option<**Vec<String>**> | List of certificate IDs to bind on the frontend | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


