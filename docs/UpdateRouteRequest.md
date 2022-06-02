# UpdateRouteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Route name | [optional]
**topic** | Option<**String**> | Topic the route subscribes to. It must be a valid MQTT topic and up to 65535 characters | [optional]
**s3_config** | Option<[**crate::models::UpdateRouteRequestS3Config**](UpdateRoute_request_s3_config.md)> |  | [optional]
**db_config** | Option<[**crate::models::UpdateRouteRequestDbConfig**](UpdateRoute_request_db_config.md)> |  | [optional]
**rest_config** | Option<[**crate::models::UpdateRouteRequestRestConfig**](UpdateRoute_request_rest_config.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


