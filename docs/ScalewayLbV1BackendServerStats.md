# ScalewayLbV1BackendServerStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instance_id** | Option<**String**> | ID of your loadbalancer cluster server | [optional]
**backend_id** | Option<**String**> | ID of your Backend | [optional]
**ip** | Option<**String**> | IPv4 or IPv6 address of the server backend | [optional]
**server_state** | Option<**String**> | Server operational state (stopped/starting/running/stopping) | [optional][default to ServerState_Stopped]
**server_state_changed_at** | Option<**String**> | Time since last operational change | [optional]
**last_health_check_status** | Option<**String**> | Last health check status (unknown/neutral/failed/passed/condpass) | [optional][default to LastHealthCheckStatus_Unknown]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


