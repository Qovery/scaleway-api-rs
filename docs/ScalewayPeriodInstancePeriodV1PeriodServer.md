# ScalewayPeriodInstancePeriodV1PeriodServer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The server unique ID | [optional]
**name** | Option<**String**> | The server name | [optional]
**organization** | Option<**String**> | The server organization ID | [optional]
**project** | Option<**String**> | The server project ID | [optional]
**allowed_actions** | Option<[**Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodServerPeriodAction>**](scaleway.instance.v1.Server.Action.md)> | Provide as list of allowed actions on the server | [optional]
**tags** | Option<**Vec<String>**> | The server associated tags | [optional]
**commercial_type** | Option<**String**> | The server commercial type (eg. GP1-M) | [optional]
**creation_date** | Option<**String**> | The server creation date (RFC 3339 format) | [optional]
**dynamic_ip_required** | Option<**bool**> | True if a dynamic IP is required | [optional]
**enable_ipv6** | Option<**bool**> | True if IPv6 is enabled | [optional]
**hostname** | Option<**String**> | The server host name | [optional]
**image** | Option<[**crate::models::ScalewayInstanceV1ServerImage**](scaleway_instance_v1_Server_image.md)> |  | [optional]
**protected** | Option<**bool**> | The server protection option is activated | [optional]
**private_ip** | Option<**String**> | The server private IP address | [optional]
**public_ip** | Option<[**crate::models::ScalewayInstanceV1ServerPublicIp**](scaleway_instance_v1_Server_public_ip.md)> |  | [optional]
**modification_date** | Option<**String**> | The server modification date (RFC 3339 format) | [optional]
**state** | Option<**String**> | The server state | [optional][default to Running]
**location** | Option<[**crate::models::ScalewayInstanceV1ServerLocation**](scaleway_instance_v1_Server_location.md)> |  | [optional]
**ipv6** | Option<[**crate::models::ScalewayInstanceV1ServerIpv6**](scaleway_instance_v1_Server_ipv6.md)> |  | [optional]
**bootscript** | Option<[**crate::models::ScalewayInstanceV1ServerBootscript**](scaleway_instance_v1_Server_bootscript.md)> |  | [optional]
**boot_type** | Option<**String**> | The server boot type | [optional][default to Local]
**volumes** | Option<[**crate::models::ScalewayInstanceV1ServerVolumes**](scaleway_instance_v1_Server_volumes.md)> |  | [optional]
**security_group** | Option<[**crate::models::ScalewayInstanceV1ServerSecurityGroup**](scaleway_instance_v1_Server_security_group.md)> |  | [optional]
**maintenances** | Option<[**Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodServerPeriodMaintenance>**](scaleway.instance.v1.Server.Maintenance.md)> | The server planned maintenances | [optional]
**state_detail** | Option<**String**> | The server state_detail | [optional]
**arch** | Option<**String**> | The server arch | [optional][default to X8664]
**placement_group** | Option<[**crate::models::ScalewayInstanceV1ServerPlacementGroup**](scaleway_instance_v1_Server_placement_group.md)> |  | [optional]
**private_nics** | Option<[**Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodPrivateNic>**](scaleway.instance.v1.PrivateNIC.md)> | The server private NICs | [optional]
**zone** | Option<**String**> | The zone in which is the server | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


