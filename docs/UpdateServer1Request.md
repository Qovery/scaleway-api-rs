# UpdateServer1Request

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the server | [optional]
**boot_type** | Option<[**crate::models::ScalewayPeriodInstancePeriodV1PeriodBootType**](scaleway.instance.v1.BootType.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | Tags of the server | [optional]
**volumes** | Option<[**crate::models::UpdateServer1RequestVolumes**](UpdateServer1_request_volumes.md)> |  | [optional]
**bootscript** | Option<**String**> |  | [optional]
**dynamic_ip_required** | Option<**bool**> |  | [optional]
**enable_ipv6** | Option<**bool**> |  | [optional]
**protected** | Option<**bool**> |  | [optional]
**security_group** | Option<[**crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroupTemplate**](scaleway.instance.v1.SecurityGroupTemplate.md)> |  | [optional]
**placement_group** | Option<**String**> | Placement group ID if server must be part of a placement group | [optional]
**private_nics** | Option<[**Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodPrivateNic>**](scaleway.instance.v1.PrivateNIC.md)> | The server private NICs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


