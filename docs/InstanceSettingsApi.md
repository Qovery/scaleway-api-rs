# \InstanceSettingsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_instance_settings**](InstanceSettingsApi.md#add_instance_settings) | **post** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Add an instance setting
[**delete_instance_settings**](InstanceSettingsApi.md#delete_instance_settings) | **delete** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Delete an instance setting
[**set_instance_settings**](InstanceSettingsApi.md#set_instance_settings) | **put** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Set a given instance setting



## add_instance_settings

> crate::models::ScalewayRdbV1AddInstanceSettingsResponse add_instance_settings(region, instance_id, inline_object68)
Add an instance setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to add settings to | [required] |
**inline_object68** | [**InlineObject68**](InlineObject68.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1AddInstanceSettingsResponse**](scaleway.rdb.v1.AddInstanceSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance_settings

> crate::models::ScalewayRdbV1DeleteInstanceSettingsResponse delete_instance_settings(region, instance_id, inline_object69)
Delete an instance setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance to delete settings from | [required] |
**inline_object69** | [**InlineObject69**](InlineObject69.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1DeleteInstanceSettingsResponse**](scaleway.rdb.v1.DeleteInstanceSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_instance_settings

> crate::models::ScalewayRdbV1SetInstanceSettingsResponse set_instance_settings(region, instance_id, inline_object67)
Set a given instance setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance where the settings has to be set | [required] |
**inline_object67** | [**InlineObject67**](InlineObject67.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1SetInstanceSettingsResponse**](scaleway.rdb.v1.SetInstanceSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

