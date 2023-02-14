# \DatabaseInstancesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_instance**](DatabaseInstancesApi.md#clone_instance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/clone | Clone an instance
[**create_instance**](DatabaseInstancesApi.md#create_instance) | **POST** /rdb/v1/regions/{region}/instances | Create an instance
[**delete_instance**](DatabaseInstancesApi.md#delete_instance) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id} | Delete an instance
[**get_instance**](DatabaseInstancesApi.md#get_instance) | **GET** /rdb/v1/regions/{region}/instances/{instance_id} | Get an instance
[**get_instance_certificate**](DatabaseInstancesApi.md#get_instance_certificate) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/certificate | Get the TLS certificate of an instance
[**get_instance_log**](DatabaseInstancesApi.md#get_instance_log) | **GET** /rdb/v1/regions/{region}/logs/{instance_log_id} | Get specific logs of a given instance
[**get_instance_metrics**](DatabaseInstancesApi.md#get_instance_metrics) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/metrics | Get instance metrics
[**list_instance_logs**](DatabaseInstancesApi.md#list_instance_logs) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/logs | List available logs of a given instance
[**list_instance_logs_details**](DatabaseInstancesApi.md#list_instance_logs_details) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/logs-details | List remote instances logs details
[**list_instances**](DatabaseInstancesApi.md#list_instances) | **GET** /rdb/v1/regions/{region}/instances | List instances
[**prepare_instance_logs**](DatabaseInstancesApi.md#prepare_instance_logs) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/prepare-logs | Prepare logs of a given instance
[**purge_instance_logs**](DatabaseInstancesApi.md#purge_instance_logs) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/purge-logs | Purge remote instances logs
[**renew_instance_certificate**](DatabaseInstancesApi.md#renew_instance_certificate) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/renew-certificate | Renew the TLS certificate of an instance
[**restart_instance**](DatabaseInstancesApi.md#restart_instance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/restart | Restart an instance
[**update_instance**](DatabaseInstancesApi.md#update_instance) | **PATCH** /rdb/v1/regions/{region}/instances/{instance_id} | Update an instance
[**upgrade_instance**](DatabaseInstancesApi.md#upgrade_instance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/upgrade | Upgrade an instance



## clone_instance

> crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance clone_instance(region, instance_id, clone_instance_request)
Clone an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to clone | [required] |
**clone_instance_request** | [**CloneInstanceRequest**](CloneInstanceRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance create_instance(region, create_instance_request)
Create an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**create_instance_request** | [**CreateInstanceRequest**](CreateInstanceRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance

> crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance delete_instance(region, instance_id)
Delete an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance to delete | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance

> crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance get_instance(region, instance_id)
Get an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_certificate

> crate::models::ScalewayPeriodStdPeriodFile get_instance_certificate(region, instance_id)
Get the TLS certificate of an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |

### Return type

[**crate::models::ScalewayPeriodStdPeriodFile**](scaleway.std.File.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_log

> crate::models::ScalewayPeriodRdbPeriodV1PeriodInstanceLog get_instance_log(region, instance_log_id)
Get specific logs of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_log_id** | **String** | UUID of the instance_log you want | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodInstanceLog**](scaleway.rdb.v1.InstanceLog.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_metrics

> crate::models::ScalewayPeriodRdbPeriodV1PeriodInstanceMetrics get_instance_metrics(region, instance_id, start_date, end_date, metric_name)
Get instance metrics

Get database instance metrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**start_date** | Option<**String**> | Start date to gather metrics from (RFC 3339 format) |  |
**end_date** | Option<**String**> | End date to gather metrics from (RFC 3339 format) |  |
**metric_name** | Option<**String**> | Name of the metric to gather |  |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodInstanceMetrics**](scaleway.rdb.v1.InstanceMetrics.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instance_logs

> crate::models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsResponse list_instance_logs(region, instance_id, order_by)
List available logs of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want logs of | [required] |
**order_by** | Option<**String**> | Criteria to use when ordering instance logs listing |  |[default to created_at_asc]

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsResponse**](scaleway.rdb.v1.ListInstanceLogsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instance_logs_details

> crate::models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse list_instance_logs_details(region, instance_id)
List remote instances logs details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want logs of | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse**](scaleway.rdb.v1.ListInstanceLogsDetailsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instances

> crate::models::ScalewayPeriodRdbPeriodV1PeriodListInstancesResponse list_instances(region, tags, name, order_by, organization_id, project_id, page, page_size)
List instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**tags** | Option<[**Vec<String>**](String.md)> | List instance that have a given tags |  |
**name** | Option<**String**> | List instance that match a given name pattern |  |
**order_by** | Option<**String**> | Criteria to use when ordering instance listing |  |[default to created_at_asc]
**organization_id** | Option<**String**> | Please use `project_id` instead |  |
**project_id** | Option<**String**> | Project ID to list the instance of |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodListInstancesResponse**](scaleway.rdb.v1.ListInstancesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_instance_logs

> crate::models::ScalewayPeriodRdbPeriodV1PeriodPrepareInstanceLogsResponse prepare_instance_logs(region, instance_id, prepare_instance_logs_request)
Prepare logs of a given instance

Prepare your instance logs. Logs will be grouped on a minimum interval of a day.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want logs of | [required] |
**prepare_instance_logs_request** | [**PrepareInstanceLogsRequest**](PrepareInstanceLogsRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodPrepareInstanceLogsResponse**](scaleway.rdb.v1.PrepareInstanceLogsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_instance_logs

> purge_instance_logs(region, instance_id, purge_instance_logs_request)
Purge remote instances logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want logs of | [required] |
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
Renew the TLS certificate of an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want logs of | [required] |
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

> crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance restart_instance(region, instance_id, body)
Restart an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to restart | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance

> crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance update_instance(region, instance_id, update_instance_request)
Update an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance to update | [required] |
**update_instance_request** | [**UpdateInstanceRequest**](UpdateInstanceRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_instance

> crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance upgrade_instance(region, instance_id, upgrade_instance_request)
Upgrade an instance

Upgrade your current instance specifications like node type, high availability, volume, or db engine version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to upgrade | [required] |
**upgrade_instance_request** | [**UpgradeInstanceRequest**](UpgradeInstanceRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

