# ScalewayBaremetalV1Server

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the server | [optional]
**organization_id** | Option<**String**> | Organization ID the server is attached to | [optional]
**project_id** | Option<**String**> | Project ID the server is attached to | [optional]
**name** | Option<**String**> | Name of the server | [optional]
**description** | Option<**String**> | Description of the server | [optional]
**updated_at** | Option<**String**> | Date of last modification of the server | [optional]
**created_at** | Option<**String**> | Date of creation of the server | [optional]
**status** | Option<**String**> | Status of the server | [optional][default to Status_Unknown]
**offer_id** | Option<**String**> | Offer ID of the server | [optional]
**tags** | Option<**Vec<String>**> | Array of customs tags attached to the server | [optional]
**ips** | Option<[**Vec<crate::models::ScalewayBaremetalV1Ip>**](scaleway.baremetal.v1.IP.md)> | Array of IPs attached to the server | [optional]
**domain** | Option<**String**> | Domain of the server | [optional]
**boot_type** | Option<**String**> | Boot type of the server | [optional][default to BootType_UnknownBootType]
**zone** | Option<**String**> | The zone in which is the server | [optional]
**install** | Option<[**crate::models::ScalewayBaremetalV1ServerInstall**](scaleway_baremetal_v1_Server_install.md)> |  | [optional]
**ping_status** | Option<**String**> | Server status of ping | [optional][default to PingStatus_Unknown]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


