# CreateServer1Request

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The server name | 
**dynamic_ip_required** | Option<**bool**> | Define if a dynamic IP is required for the instance | [optional]
**commercial_type** | **String** | Define the server commercial type (i.e. GP1-S) | 
**image** | Option<**String**> | The server image ID | [optional]
**volumes** | Option<[**crate::models::CreateServer1RequestVolumes**](CreateServer1_request_volumes.md)> |  | [optional]
**enable_ipv6** | Option<**bool**> | True if IPv6 is enabled on the server | [optional]
**public_ip** | Option<**String**> | The ID of the reserved IP to attach to the server | [optional]
**boot_type** | Option<**String**> | The boot type to use | [optional][default to Local]
**bootscript** | Option<**String**> | The bootscript ID to use when `boot_type` is set to `bootscript` | [optional]
**organization** | Option<**String**> | The server organization ID | [optional]
**project** | Option<**String**> | The server project ID | [optional]
**tags** | Option<**Vec<String>**> | The server tags | [optional]
**security_group** | Option<**String**> | The security group ID | [optional]
**placement_group** | Option<**String**> | Placement group ID if server must be part of a placement group | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


