# CreateEndpointRequestEndpointSpecPrivateNetwork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**private_network_id** | Option<**String**> | UUID of the Private Network to be connected to the Database Instance. (UUID format) | [optional]
**service_ip** | Option<**String**> | Endpoint IPv4 address with a CIDR notation. Refer to the official Scaleway documentation to learn more about IP and subnet limitations. (IP network) | [optional]
**ipam_config** | Option<[**serde_json::Value**](.md)> | Automated configuration of your Private Network endpoint with Scaleway IPAM service. One at the most per Database Instance or Read Replica (a Database Instance and its Read Replica can have different Private Networks). Cannot be updated (has to be deleted and recreated). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


