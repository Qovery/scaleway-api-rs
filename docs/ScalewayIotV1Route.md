# ScalewayIotV1Route

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Route ID | [optional]
**name** | Option<**String**> | Route name | [optional]
**hub_id** | Option<**String**> | ID of the route's hub | [optional]
**topic** | Option<**String**> | Topic the route subscribes to. It must be a valid MQTT topic and up to 65535 characters | [optional]
**_type** | Option<**String**> | Route type | [optional][default to Type_Unknown]
**created_at** | Option<**String**> | Route creation date | [optional]
**s3_config** | Option<[**crate::models::ScalewayIotV1RouteS3Config**](scaleway_iot_v1_Route_s3_config.md)> |  | [optional]
**db_config** | Option<[**crate::models::ScalewayIotV1RouteDbConfig**](scaleway_iot_v1_Route_db_config.md)> |  | [optional]
**rest_config** | Option<[**crate::models::ScalewayIotV1RouteRestConfig**](scaleway_iot_v1_Route_rest_config.md)> |  | [optional]
**updated_at** | Option<**String**> | Route last update date | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


