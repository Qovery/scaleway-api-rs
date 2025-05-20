# \DatabaseInstancesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_instance_maintenance**](DatabaseInstancesApi.md#apply_instance_maintenance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/apply-maintenance | Apply Database Instance maintenance
[**clone_instance**](DatabaseInstancesApi.md#clone_instance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/clone | Clone a Database Instance
[**create_instance**](DatabaseInstancesApi.md#create_instance) | **POST** /rdb/v1/regions/{region}/instances | Create a Database Instance
[**delete_instance**](DatabaseInstancesApi.md#delete_instance) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id} | Delete a Database Instance
[**get_instance**](DatabaseInstancesApi.md#get_instance) | **GET** /rdb/v1/regions/{region}/instances/{instance_id} | Get a Database Instance
[**get_instance_certificate**](DatabaseInstancesApi.md#get_instance_certificate) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/certificate | Get the TLS certificate of a Database Instance
[**get_instance_log**](DatabaseInstancesApi.md#get_instance_log) | **GET** /rdb/v1/regions/{region}/logs/{instance_log_id} | Get given logs of a Database Instance
[**get_instance_metrics**](DatabaseInstancesApi.md#get_instance_metrics) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/metrics | Get Database Instance metrics
[**list_instance_logs**](DatabaseInstancesApi.md#list_instance_logs) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/logs | List available logs of a Database Instance
[**list_instance_logs_details**](DatabaseInstancesApi.md#list_instance_logs_details) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/logs-details | List remote Database Instance logs details
[**list_instances**](DatabaseInstancesApi.md#list_instances) | **GET** /rdb/v1/regions/{region}/instances | List Database Instances
[**prepare_instance_logs**](DatabaseInstancesApi.md#prepare_instance_logs) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/prepare-logs | Prepare logs of a Database Instance
[**purge_instance_logs**](DatabaseInstancesApi.md#purge_instance_logs) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/purge-logs | Purge remote Database Instance logs
[**renew_instance_certificate**](DatabaseInstancesApi.md#renew_instance_certificate) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/renew-certificate | Renew the TLS certificate of a Database Instance
[**restart_instance**](DatabaseInstancesApi.md#restart_instance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/restart | Restart Database Instance
[**update_instance**](DatabaseInstancesApi.md#update_instance) | **PATCH** /rdb/v1/regions/{region}/instances/{instance_id} | Update a Database Instance
[**upgrade_instance**](DatabaseInstancesApi.md#upgrade_instance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/upgrade | Upgrade a Database Instance



## apply_instance_maintenance

> models::ScalewayPeriodRdbPeriodV1PeriodMaintenance apply_instance_maintenance(region, instance_id, body)
Apply Database Instance maintenance

Apply maintenance tasks to your Database Instance. This will trigger pending maintenance tasks to start in your Database Instance and can generate service interruption. Maintenance tasks can be applied between `starts_at` and `stops_at` times, and are run directly by Scaleway at `forced_at` timestamp.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want to apply maintenance. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodMaintenance**](scaleway.rdb.v1.Maintenance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clone_instance

> models::ScalewayPeriodRdbPeriodV1PeriodInstance clone_instance(region, instance_id, clone_instance_request)
Clone a Database Instance

Clone a given Database Instance, specified by the `region` and `instance_id` parameters. The clone feature allows you to create a new Database Instance from an existing one. The clone includes all existing databases, users and permissions. You can create a clone on a Database Instance bigger than your current one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want to clone. | [required] |
**clone_instance_request** | [**CloneInstanceRequest**](CloneInstanceRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> models::ScalewayPeriodRdbPeriodV1PeriodInstance create_instance(region, create_instance_request)
Create a Database Instance

Create a new Database Instance. You must set the `engine`, `user_name`, `password` and `node_type` parameters. Optionally, you can specify the volume type and size.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**create_instance_request** | [**CreateInstanceRequest**](CreateInstanceRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance

> models::ScalewayPeriodRdbPeriodV1PeriodInstance delete_instance(region, instance_id)
Delete a Database Instance

Delete a given Database Instance, specified by the `region` and `instance_id` parameters. Deleting a Database Instance is permanent, and cannot be undone. Note that upon deletion all your data will be lost.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance to delete. | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance

> models::ScalewayPeriodRdbPeriodV1PeriodInstance get_instance(region, instance_id)
Get a Database Instance

Retrieve information about a given Database Instance, specified by the `region` and `instance_id` parameters. Its full details, including name, status, IP address and port, are returned in the response object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance. | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_certificate

> models::ScalewayPeriodStdPeriodFile get_instance_certificate(region, instance_id)
Get the TLS certificate of a Database Instance

Retrieve information about the TLS certificate of a given Database Instance. Details like name and content are returned in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance. | [required] |

### Return type

[**models::ScalewayPeriodStdPeriodFile**](scaleway.std.File.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_log

> models::ScalewayPeriodRdbPeriodV1PeriodInstanceLog get_instance_log(region, instance_log_id)
Get given logs of a Database Instance

Retrieve information about the logs of a Database Instance. Specify the `instance_log_id` and `region` in your request to get information such as `download_url`, `status`, `expires_at` and `created_at` about your logs in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_log_id** | **String** | UUID of the instance_log you want. | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstanceLog**](scaleway.rdb.v1.InstanceLog.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_metrics

> models::ScalewayPeriodRdbPeriodV1PeriodInstanceMetrics get_instance_metrics(region, instance_id, start_date, end_date, metric_name)
Get Database Instance metrics

Retrieve the time series metrics of a given Database Instance. You can define the period from which to retrieve metrics by specifying the `start_date` and `end_date`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance. | [required] |
**start_date** | Option<**String**> | Start date to gather metrics from. (RFC 3339 format) |  |
**end_date** | Option<**String**> | End date to gather metrics from. (RFC 3339 format) |  |
**metric_name** | Option<**String**> | Name of the metric to gather. |  |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstanceMetrics**](scaleway.rdb.v1.InstanceMetrics.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instance_logs

> models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsResponse list_instance_logs(region, instance_id, order_by)
List available logs of a Database Instance

List the available logs of a Database Instance. By default, the logs returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want logs of. | [required] |
**order_by** | Option<**String**> | Criteria to use when ordering Database Instance logs listing. |  |[default to created_at_asc]

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsResponse**](scaleway.rdb.v1.ListInstanceLogsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instance_logs_details

> models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse list_instance_logs_details(region, instance_id)
List remote Database Instance logs details

List remote log details. By default, the details returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want logs of. | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse**](scaleway.rdb.v1.ListInstanceLogsDetailsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instances

> models::ScalewayPeriodRdbPeriodV1PeriodListInstancesResponse list_instances(region, tags, name, order_by, organization_id, project_id, page, page_size)
List Database Instances

List all Database Instances in the specified region, for a given Scaleway Organization or Scaleway Project. By default, the Database Instances returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field. You can define additional parameters for your query, such as `tags` and `name`. For the `name` parameter, the value you include will be checked against the whole name string to see if it includes the string you put in the parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**tags** | Option<[**Vec<String>**](String.md)> | List Database Instances that have a given tag. |  |
**name** | Option<**String**> | Lists Database Instances that match a name pattern. |  |
**order_by** | Option<**String**> | Criteria to use when ordering Database Instance listings. |  |[default to created_at_asc]
**organization_id** | Option<**String**> | Please use project_id instead. |  |
**project_id** | Option<**String**> | Project ID to list the Database Instance of. |  |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodListInstancesResponse**](scaleway.rdb.v1.ListInstancesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_instance_logs

> models::ScalewayPeriodRdbPeriodV1PeriodPrepareInstanceLogsResponse prepare_instance_logs(region, instance_id, prepare_instance_logs_request)
Prepare logs of a Database Instance

Prepare your Database Instance logs. You can define the `start_date` and `end_date` parameters for your query. The download URL is returned in the response. Logs are recorded from 00h00 to 23h59 and then aggregated in a `.log` file once a day. Therefore, even if you specify a timeframe from which you want to get the logs, you will receive logs from the full 24 hours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want logs of. | [required] |
**prepare_instance_logs_request** | [**PrepareInstanceLogsRequest**](PrepareInstanceLogsRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodPrepareInstanceLogsResponse**](scaleway.rdb.v1.PrepareInstanceLogsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_instance_logs

> purge_instance_logs(region, instance_id, purge_instance_logs_request)
Purge remote Database Instance logs

Purge a given remote log from a Database Instance. You can specify the `log_name` of the log you wish to clean from your Database Instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want logs of. | [required] |
**purge_instance_logs_request** | [**PurgeInstanceLogsRequest**](PurgeInstanceLogsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## renew_instance_certificate

> renew_instance_certificate(region, instance_id, body)
Renew the TLS certificate of a Database Instance

Renew a TLS for a Database Instance. Renewing a certificate means that you will not be able to connect to your Database Instance using the previous certificate. You will also need to download and update the new certificate for all database clients.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want logs of. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_instance

> models::ScalewayPeriodRdbPeriodV1PeriodInstance restart_instance(region, instance_id, body)
Restart Database Instance

Restart a given Database Instance, specified by the `region` and `instance_id` parameters. The status of the Database Instance returned in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want to restart. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance

> models::ScalewayPeriodRdbPeriodV1PeriodInstance update_instance(region, instance_id, update_instance_request)
Update a Database Instance

Update the parameters of a Database Instance, including name, tags and backup schedule details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance to update. | [required] |
**update_instance_request** | [**UpdateInstanceRequest**](UpdateInstanceRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_instance

> models::ScalewayPeriodRdbPeriodV1PeriodInstance upgrade_instance(region, instance_id, upgrade_instance_request)
Upgrade a Database Instance

Upgrade your current Database Instance specifications like node type, high availability, volume, or the database engine version. Note that upon upgrade the `enable_ha` parameter can only be set to `true`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want to upgrade. | [required] |
**upgrade_instance_request** | [**UpgradeInstanceRequest**](UpgradeInstanceRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

