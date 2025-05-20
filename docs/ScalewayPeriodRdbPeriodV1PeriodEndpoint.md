# ScalewayPeriodRdbPeriodV1PeriodEndpoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the endpoint. (UUID format) | [optional]
**ip** | Option<**String**> | IPv4 address of the endpoint. (IP address) | [optional]
**port** | Option<**i32**> | TCP port of the endpoint. | [optional]
**name** | Option<**String**> | Name of the endpoint. | [optional]
**private_network** | Option<[**models::ScalewayRdbV1EndpointPrivateNetwork**](scaleway_rdb_v1_Endpoint_private_network.md)> |  | [optional]
**load_balancer** | Option<[**serde_json::Value**](.md)> | Load balancer details. Public endpoint for Database Instance which is systematically present. One per Database Instance. | [optional]
**direct_access** | Option<[**serde_json::Value**](.md)> | Direct access details. Public endpoint reserved for Read Replicas. One per Read Replica. | [optional]
**hostname** | Option<**String**> | Hostname of the endpoint. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


