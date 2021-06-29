# InlineObject31

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**port** | **f32** | Specify the port used to health check | 
**check_delay** | **f32** | Time between two consecutive health checks (in milliseconds) | 
**check_timeout** | **f32** | Additional check timeout, after the connection has been already established (in milliseconds) | 
**check_max_retries** | **f32** | Number of consecutive unsuccessful health checks, after wich the server will be considered dead | 
**mysql_config** | Option<[**crate::models::LbV1RegionsRegionBackendsBackendIdHealthcheckMysqlConfig**](_lb_v1_regions__region__backends__backend_id__healthcheck_mysql_config.md)> |  | [optional]
**ldap_config** | Option<[**serde_json::Value**](.md)> | The response is analyzed to find an LDAPv3 response message | [optional]
**redis_config** | Option<[**serde_json::Value**](.md)> | The response is analyzed to find the +PONG response message | [optional]
**pgsql_config** | Option<[**crate::models::LbV1RegionsRegionBackendsBackendIdHealthcheckPgsqlConfig**](_lb_v1_regions__region__backends__backend_id__healthcheck_pgsql_config.md)> |  | [optional]
**tcp_config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**http_config** | Option<[**crate::models::LbV1RegionsRegionBackendsBackendIdHealthcheckHttpConfig**](_lb_v1_regions__region__backends__backend_id__healthcheck_http_config.md)> |  | [optional]
**https_config** | Option<[**crate::models::LbV1RegionsRegionBackendsBackendIdHealthcheckHttpConfig**](_lb_v1_regions__region__backends__backend_id__healthcheck_http_config.md)> |  | [optional]
**check_send_proxy** | Option<**bool**> | It defines whether the healthcheck should be done considering the proxy protocol | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


