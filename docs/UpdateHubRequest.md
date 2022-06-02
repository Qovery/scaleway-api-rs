# UpdateHubRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Hub name (up to 255 characters) | [optional]
**product_plan** | Option<**String**> | Hub feature set | [optional][default to ProductPlan_Unknown]
**disable_events** | Option<**bool**> | Disable Hub events | [optional]
**events_topic_prefix** | Option<**String**> | Hub events topic prefix | [optional]
**enable_device_auto_provisioning** | Option<**bool**> | Enable device auto provisioning | [optional]
**twins_graphite_config** | Option<[**crate::models::CreateHubRequestTwinsGraphiteConfig**](CreateHub_request_twins_graphite_config.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


