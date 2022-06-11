# \InstanceSettingsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_instance_settings**](InstanceSettingsApi.md#add_instance_settings) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Add an instance setting
[**delete_instance_settings**](InstanceSettingsApi.md#delete_instance_settings) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Delete an instance setting
[**set_instance_settings**](InstanceSettingsApi.md#set_instance_settings) | **PUT** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Set a given instance setting



## add_instance_settings

> crate::models::ScalewayRdbV1AddInstanceSettingsResponse add_instance_settings(region, instance_id, add_instance_settings_request)
Add an instance setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to add settings to | [required] |
**add_instance_settings_request** | [**AddInstanceSettingsRequest**](AddInstanceSettingsRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1AddInstanceSettingsResponse**](scaleway.rdb.v1.AddInstanceSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance_settings

> crate::models::ScalewayRdbV1DeleteInstanceSettingsResponse delete_instance_settings(region, instance_id, delete_instance_settings_request)
Delete an instance setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance to delete settings from | [required] |
**delete_instance_settings_request** | [**DeleteInstanceSettingsRequest**](DeleteInstanceSettingsRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1DeleteInstanceSettingsResponse**](scaleway.rdb.v1.DeleteInstanceSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_instance_settings

> crate::models::ScalewayRdbV1SetInstanceSettingsResponse set_instance_settings(region, instance_id, set_instance_settings_request)
Set a given instance setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance where the settings has to be set | [required] |
**set_instance_settings_request** | [**SetInstanceSettingsRequest**](SetInstanceSettingsRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1SetInstanceSettingsResponse**](scaleway.rdb.v1.SetInstanceSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

