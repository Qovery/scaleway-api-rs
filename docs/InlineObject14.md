# InlineObject14

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the server | [optional]
**boot_type** | Option<[**crate::models::ScalewayInstanceV1BootType**](scaleway.instance.v1.BootType.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | Tags of the server | [optional]
**volumes** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**bootscript** | Option<**String**> |  | [optional]
**dynamic_ip_required** | Option<**bool**> |  | [optional]
**enable_ipv6** | Option<**bool**> |  | [optional]
**protected** | Option<**bool**> |  | [optional]
**security_group** | Option<[**crate::models::ScalewayInstanceV1SecurityGroupTemplate**](scaleway.instance.v1.SecurityGroupTemplate.md)> |  | [optional]
**placement_group** | Option<**String**> | Placement group ID if server must be part of a placement group | [optional]
**private_nics** | Option<[**Vec<crate::models::ScalewayInstanceV1PrivateNic>**](scaleway.instance.v1.PrivateNIC.md)> | The server private NICs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


