# ScalewayPeriodBaremetalPeriodV1PeriodServer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the server. | [optional]
**organization_id** | Option<**String**> | Organization ID the server is attached to. | [optional]
**project_id** | Option<**String**> | Project ID the server is attached to. | [optional]
**name** | Option<**String**> | Name of the server. | [optional]
**description** | Option<**String**> | Description of the server. | [optional]
**updated_at** | Option<**String**> | Last modification date of the server. (RFC 3339 format) | [optional]
**created_at** | Option<**String**> | Creation date of the server. (RFC 3339 format) | [optional]
**status** | Option<**String**> | Status of the server. | [optional][default to Unknown]
**offer_id** | Option<**String**> | Offer ID of the server. | [optional]
**offer_name** | Option<**String**> | Offer name of the server. | [optional]
**tags** | Option<**Vec<String>**> | Array of custom tags attached to the server. | [optional]
**ips** | Option<[**Vec<models::ScalewayPeriodBaremetalPeriodV1PeriodIp>**](scaleway.baremetal.v1.IP.md)> | Array of IPs attached to the server. | [optional]
**domain** | Option<**String**> | Domain of the server. | [optional]
**boot_type** | Option<**String**> | Boot type of the server. | [optional][default to UnknownBootType]
**zone** | Option<**String**> | Zone in which is the server located. | [optional]
**install** | Option<[**models::ScalewayBaremetalV1ServerInstall**](scaleway_baremetal_v1_Server_install.md)> |  | [optional]
**ping_status** | Option<**String**> | Status of server ping. | [optional][default to PingStatusUnknown]
**options** | Option<[**Vec<models::ScalewayPeriodBaremetalPeriodV1PeriodServerPeriodOption>**](scaleway.baremetal.v1.Server.Option.md)> | Options enabled on the server. | [optional]
**rescue_server** | Option<[**models::ScalewayBaremetalV1ServerRescueServer**](scaleway_baremetal_v1_Server_rescue_server.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


