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
[**list_instances**](DatabaseInstancesApi.md#list_instances) | **GET** /rdb/v1/regions/{region}/instances | List instances
[**prepare_instance_logs**](DatabaseInstancesApi.md#prepare_instance_logs) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/prepare-logs | Prepare logs of a given instance
[**update_instance**](DatabaseInstancesApi.md#update_instance) | **PATCH** /rdb/v1/regions/{region}/instances/{instance_id} | Update an instance
[**upgrade_instance**](DatabaseInstancesApi.md#upgrade_instance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/upgrade | Upgrade an instance to an higher instance type



## clone_instance

> crate::models::ScalewayRdbV1Instance clone_instance(region, instance_id, inline_object15)
Clone an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to clone | [required] |
**inline_object15** | [**InlineObject15**](InlineObject15.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Instance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> crate::models::ScalewayRdbV1Instance create_instance(region, inline_object10)
Create an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**inline_object10** | [**InlineObject10**](InlineObject10.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Instance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance

> crate::models::ScalewayRdbV1Instance delete_instance(region, instance_id)
Delete an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance to delete | [required] |

### Return type

[**crate::models::ScalewayRdbV1Instance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance

> crate::models::ScalewayRdbV1Instance get_instance(region, instance_id)
Get an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |

### Return type

[**crate::models::ScalewayRdbV1Instance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_certificate

> crate::models::ScalewayStdFile get_instance_certificate(region, instance_id)
Get the TLS certificate of an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |

### Return type

[**crate::models::ScalewayStdFile**](scaleway.std.File.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_log

> crate::models::ScalewayRdbV1InstanceLog get_instance_log(region, instance_log_id)
Get specific logs of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_log_id** | **String** | UUID of the instance_log you want | [required] |

### Return type

[**crate::models::ScalewayRdbV1InstanceLog**](scaleway.rdb.v1.InstanceLog.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_metrics

> crate::models::ScalewayRdbV1InstanceMetrics get_instance_metrics(region, instance_id, start_date, end_date, metric_name)
Get instance metrics

Get database instance metrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**start_date** | Option<**String**> | Start date to gather metrics from |  |
**end_date** | Option<**String**> | End date to gather metrics from |  |
**metric_name** | Option<**String**> | Name of the metric to gather |  |

### Return type

[**crate::models::ScalewayRdbV1InstanceMetrics**](scaleway.rdb.v1.InstanceMetrics.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instance_logs

> crate::models::ScalewayRdbV1ListInstanceLogsResponse list_instance_logs(region, instance_id, order_by)
List available logs of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want logs of | [required] |
**order_by** | Option<**String**> | Criteria to use when ordering instance logs listing |  |[default to created_at_asc]

### Return type

[**crate::models::ScalewayRdbV1ListInstanceLogsResponse**](scaleway.rdb.v1.ListInstanceLogsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instances

> crate::models::ScalewayRdbV1ListInstancesResponse list_instances(region, tags, name, order_by, organization_id, project_id, page, page_size)
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
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayRdbV1ListInstancesResponse**](scaleway.rdb.v1.ListInstancesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_instance_logs

> crate::models::ScalewayRdbV1PrepareInstanceLogsResponse prepare_instance_logs(region, instance_id, inline_object17)
Prepare logs of a given instance

Prepare your instance logs. Logs will be grouped on a minimum interval of a day.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want logs of | [required] |
**inline_object17** | [**InlineObject17**](InlineObject17.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1PrepareInstanceLogsResponse**](scaleway.rdb.v1.PrepareInstanceLogsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance

> crate::models::ScalewayRdbV1Instance update_instance(region, instance_id, inline_object11)
Update an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance to update | [required] |
**inline_object11** | [**InlineObject11**](InlineObject11.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Instance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_instance

> crate::models::ScalewayRdbV1Instance upgrade_instance(region, instance_id, inline_object23)
Upgrade an instance to an higher instance type

Upgrade your current `node_type` or enable high availability on your standalone database instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to upgrade | [required] |
**inline_object23** | [**InlineObject23**](InlineObject23.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Instance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

