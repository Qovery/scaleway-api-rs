# ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the Flexible IP | [optional]
**organization_id** | Option<**String**> | Organization ID the Flexible IP is attached to | [optional]
**project_id** | Option<**String**> | Project ID the Flexible IP is attached to | [optional]
**description** | Option<**String**> | Description of the Flexible IP | [optional]
**tags** | Option<**Vec<String>**> | Tags associated with the Flexible IP | [optional]
**updated_at** | Option<**String**> | Date of last update of the Flexible IP (RFC 3339 format) | [optional]
**created_at** | Option<**String**> | Date of creation of the Flexible IP (RFC 3339 format) | [optional]
**status** | Option<**String**> | - ready : Flexible IP is created and ready to be attached to a server or to have a virtual MAC generated. - updating: Flexible IP is being attached to a server or a virtual MAC operation is ongoing - attached: Flexible IP is attached to a server - error: a Flexible IP operation resulted in an error - detaching: Flexible IP is being detached from a server - locked: Flexible IP resource is locked  | [optional][default to Unknown]
**ip_address** | Option<**String**> | IP of the Flexible IP (IP network) | [optional]
**mac_address** | Option<[**crate::models::ScalewayFlexibleIpV1alpha1FlexibleIpMacAddress**](scaleway_flexible_ip_v1alpha1_FlexibleIP_mac_address.md)> |  | [optional]
**server_id** | Option<**String**> | ID of the server linked to the Flexible IP | [optional]
**reverse** | Option<**String**> | Reverse DNS value | [optional]
**zone** | Option<**String**> | Flexible IP Availability Zone | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


