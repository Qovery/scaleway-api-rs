# ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the flexible IP. | [optional]
**organization_id** | Option<**String**> | ID of the Organization the flexible IP is attached to. | [optional]
**project_id** | Option<**String**> | ID of the Project the flexible IP is attached to. | [optional]
**description** | Option<**String**> | Flexible IP description. | [optional]
**tags** | Option<**Vec<String>**> | Flexible IP tags. | [optional]
**updated_at** | Option<**String**> | Date on which the flexible IP was last updated. (RFC 3339 format) | [optional]
**created_at** | Option<**String**> | Date on which the flexible IP was created. (RFC 3339 format) | [optional]
**status** | Option<**String**> | Flexible IP status. - ready : flexible IP is created and ready to be attached to a server or to be associated with a virtual MAC. - updating: flexible IP is being attached to a server or a virtual MAC operation is ongoing - attached: flexible IP is attached to a server - error: a flexible IP operation resulted in an error - detaching: flexible IP is being detached from a server - locked: the resource of the flexible IP is locked. | [optional][default to Unknown]
**ip_address** | Option<**String**> | IP of the flexible IP. (IP network) | [optional]
**mac_address** | Option<[**models::ScalewayFlexibleIpV1alpha1FlexibleIpMacAddress**](scaleway_flexible_ip_v1alpha1_FlexibleIP_mac_address.md)> |  | [optional]
**server_id** | Option<**String**> | ID of the server linked to the flexible IP. | [optional]
**reverse** | Option<**String**> | Reverse DNS value. | [optional]
**zone** | Option<**String**> | Availability Zone of the flexible IP. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


