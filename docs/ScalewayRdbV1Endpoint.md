# ScalewayRdbV1Endpoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the endpoint (UUID format) | [optional]
**ip** | Option<**String**> | IPv4 address of the endpoint (IP address) | [optional]
**port** | Option<**i64**> | TCP port of the endpoint | [optional]
**name** | Option<**String**> | Name of the endpoint | [optional]
**private_network** | Option<[**crate::models::ScalewayRdbV1EndpointPrivateNetwork**](scaleway_rdb_v1_Endpoint_private_network.md)> |  | [optional]
**load_balancer** | Option<[**serde_json::Value**](.md)> | Load balancer details | [optional]
**direct_access** | Option<[**serde_json::Value**](.md)> | Direct access details | [optional]
**hostname** | Option<**String**> | Hostname of the endpoint | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


