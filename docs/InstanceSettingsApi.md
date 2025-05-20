# \InstanceSettingsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_instance_settings**](InstanceSettingsApi.md#add_instance_settings) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Add Database Instance advanced settings
[**delete_instance_settings**](InstanceSettingsApi.md#delete_instance_settings) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Delete Database Instance advanced settings
[**set_instance_settings**](InstanceSettingsApi.md#set_instance_settings) | **PUT** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Set Database Instance advanced settings



## add_instance_settings

> models::ScalewayPeriodRdbPeriodV1PeriodAddInstanceSettingsResponse add_instance_settings(region, instance_id, add_instance_settings_request)
Add Database Instance advanced settings

Add an advanced setting to a Database Instance. You must set the `name` and the `value` of each setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want to add settings to. | [required] |
**add_instance_settings_request** | [**AddInstanceSettingsRequest**](AddInstanceSettingsRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodAddInstanceSettingsResponse**](scaleway.rdb.v1.AddInstanceSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance_settings

> models::ScalewayPeriodRdbPeriodV1PeriodDeleteInstanceSettingsResponse delete_instance_settings(region, instance_id, delete_instance_settings_request)
Delete Database Instance advanced settings

Delete an advanced setting in a Database Instance. You must specify the names of the settings you want to delete in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance to delete settings from. | [required] |
**delete_instance_settings_request** | [**DeleteInstanceSettingsRequest**](DeleteInstanceSettingsRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodDeleteInstanceSettingsResponse**](scaleway.rdb.v1.DeleteInstanceSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_instance_settings

> models::ScalewayPeriodRdbPeriodV1PeriodSetInstanceSettingsResponse set_instance_settings(region, instance_id, set_instance_settings_request)
Set Database Instance advanced settings

Update an advanced setting for a Database Instance. Settings added upon database engine initialization can only be defined once, and cannot, therefore, be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance where the settings must be set. | [required] |
**set_instance_settings_request** | [**SetInstanceSettingsRequest**](SetInstanceSettingsRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodSetInstanceSettingsResponse**](scaleway.rdb.v1.SetInstanceSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

