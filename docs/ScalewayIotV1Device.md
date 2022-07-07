# ScalewayIotV1Device

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Device ID, also used as MQTT Client ID or Username | [optional]
**name** | Option<**String**> | Device name | [optional]
**description** | Option<**String**> | Device description | [optional]
**status** | Option<**String**> | Device status | [optional][default to Status_Unknown]
**hub_id** | Option<**String**> | Hub ID | [optional]
**last_activity_at** | Option<**String**> | Device last connection/activity date | [optional]
**is_connected** | Option<**bool**> | Whether the device is connected to the Hub or not | [optional]
**allow_insecure** | Option<**bool**> | Whether to allow device to connect without TLS mutual authentication | [optional]
**allow_multiple_connections** | Option<**bool**> | Whether to allow multiple physical devices to connect with this device's credentials | [optional]
**message_filters** | Option<[**crate::models::UpdateDeviceRequestMessageFilters**](UpdateDevice_request_message_filters.md)> |  | [optional]
**has_custom_certificate** | Option<**bool**> | Assigning a custom certificate allows a device to authenticate using that specific certificate without checking the hub's CA certificate. | [optional]
**created_at** | Option<**String**> | Device add date | [optional]
**updated_at** | Option<**String**> | Device last modification date | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


