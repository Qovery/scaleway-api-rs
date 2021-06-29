# ScalewayLbV1PrivateNetwork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lb** | Option<[**crate::models::ScalewayLbV1PrivateNetworkLb**](scaleway_lb_v1_PrivateNetwork_lb.md)> |  | [optional]
**static_config** | Option<[**crate::models::ScalewayLbV1PrivateNetworkStaticConfig**](scaleway_lb_v1_PrivateNetwork_static_config.md)> |  | [optional]
**dhcp_config** | Option<[**serde_json::Value**](.md)> | Value set to true if load balancer instance use a DHCP | [optional]
**private_network_id** | Option<**String**> | Instance private network id | [optional]
**status** | Option<**String**> | Status (running, to create...) of private network connection | [optional][default to Status_Unknown]
**created_at** | Option<**String**> | Date at which the PN was created | [optional]
**updated_at** | Option<**String**> | Date at which the PN was last updated | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


