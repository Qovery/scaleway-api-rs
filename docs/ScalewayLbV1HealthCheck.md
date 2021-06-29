# ScalewayLbV1HealthCheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mysql_config** | Option<[**crate::models::LbV1RegionsRegionLbsLbIdBackendsHealthCheckMysqlConfig**](_lb_v1_regions__region__lbs__lb_id__backends_health_check_mysql_config.md)> |  | [optional]
**ldap_config** | Option<[**serde_json::Value**](.md)> | The response is analyzed to find an LDAPv3 response message | [optional]
**redis_config** | Option<[**serde_json::Value**](.md)> | The response is analyzed to find the +PONG response message | [optional]
**check_max_retries** | Option<**f32**> |  | [optional]
**tcp_config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**pgsql_config** | Option<[**crate::models::LbV1RegionsRegionLbsLbIdBackendsHealthCheckPgsqlConfig**](_lb_v1_regions__region__lbs__lb_id__backends_health_check_pgsql_config.md)> |  | [optional]
**http_config** | Option<[**crate::models::LbV1RegionsRegionLbsLbIdBackendsHealthCheckHttpConfig**](_lb_v1_regions__region__lbs__lb_id__backends_health_check_http_config.md)> |  | [optional]
**https_config** | Option<[**crate::models::LbV1RegionsRegionLbsLbIdBackendsHealthCheckHttpConfig**](_lb_v1_regions__region__lbs__lb_id__backends_health_check_http_config.md)> |  | [optional]
**port** | Option<**f32**> |  | [optional]
**check_timeout** | Option<**f32**> | (in milliseconds) | [optional]
**check_delay** | Option<**f32**> | (in milliseconds) | [optional]
**check_send_proxy** | Option<**bool**> | It defines whether the healthcheck should be done considering the proxy protocol | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


