# ScalewayIotV1Hub

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Hub ID | [optional]
**name** | Option<**String**> | Hub name | [optional]
**status** | Option<**String**> | Current status of the Hub | [optional][default to Status_Unknown]
**product_plan** | Option<**String**> | Hub feature set | [optional][default to ProductPlan_Unknown]
**enabled** | Option<**bool**> | Whether the hub has been enabled | [optional]
**device_count** | Option<**i64**> | Number of registered devices | [optional]
**connected_device_count** | Option<**i64**> | Number of currently connected devices | [optional]
**endpoint** | Option<**String**> | Devices should be connected to this host, port may be 1883 (MQTT), 8883 (MQTT over TLS), 80 (MQTT over Websocket) or 443 (MQTT over Websocket over TLS). | [optional]
**disable_events** | Option<**bool**> | Disable Hub events | [optional]
**events_topic_prefix** | Option<**String**> | Hub events topic prefix | [optional]
**region** | Option<**String**> | Region of the Hub | [optional]
**created_at** | Option<**String**> | Hub creation date | [optional]
**updated_at** | Option<**String**> | Hub last modification date | [optional]
**project_id** | Option<**String**> | Project owning the resource | [optional]
**organization_id** | Option<**String**> | Organization owning the resource | [optional]
**enable_device_auto_provisioning** | Option<**bool**> | When an unknown device connects to your hub using a valid certificate chain, it will be automatically provisioned inside your hub. The hub uses the common name of the device certifcate to find out if a device with the same name already exists. This setting can only be enabled on a hub with a custom certificate authority. | [optional]
**has_custom_ca** | Option<**bool**> | After creating a hub, this flag is set to False as the hub certificates are managed by Scaleway. Once a custom certificate authority is installed, this flag will be set to true. | [optional]
**twins_graphite_config** | Option<[**crate::models::CreateHubRequestTwinsGraphiteConfig**](CreateHub_request_twins_graphite_config.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


