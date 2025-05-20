# \OptionsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_option**](OptionsApi.md#get_option) | **GET** /baremetal/v1/zones/{zone}/options/{option_id} | Get option
[**list_options**](OptionsApi.md#list_options) | **GET** /baremetal/v1/zones/{zone}/options | List options
[**list_settings**](OptionsApi.md#list_settings) | **GET** /baremetal/v1/zones/{zone}/settings | List all settings
[**update_setting**](OptionsApi.md#update_setting) | **PATCH** /baremetal/v1/zones/{zone}/settings/{setting_id} | Update setting



## get_option

> models::ScalewayPeriodBaremetalPeriodV1PeriodOption get_option(zone, option_id)
Get option

Return specific option for the ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**option_id** | **String** | ID of the option. | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodOption**](scaleway.baremetal.v1.Option.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_options

> models::ScalewayPeriodBaremetalPeriodV1PeriodListOptionsResponse list_options(zone, page, page_size, offer_id, name)
List options

List all options matching with filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**page** | Option<**i32**> | Page number. |  |
**page_size** | Option<**i32**> | Number of options per page. |  |
**offer_id** | Option<**String**> | Offer ID to filter options for. |  |
**name** | Option<**String**> | Name to filter options for. |  |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodListOptionsResponse**](scaleway.baremetal.v1.ListOptionsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_settings

> models::ScalewayPeriodBaremetalPeriodV1PeriodListSettingsResponse list_settings(zone, page, page_size, order_by, project_id)
List all settings

Return all settings for a Project ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**page** | Option<**i32**> | Page number. |  |
**page_size** | Option<**i32**> | Set the maximum list size. |  |
**order_by** | Option<**String**> | Sort order for items in the response. |  |[default to created_at_asc]
**project_id** | Option<**String**> | ID of the Project. (UUID format) |  |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodListSettingsResponse**](scaleway.baremetal.v1.ListSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_setting

> models::ScalewayPeriodBaremetalPeriodV1PeriodSetting update_setting(zone, setting_id, update_setting_request)
Update setting

Update a setting for a Project ID (enable or disable).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**setting_id** | **String** | ID of the setting. | [required] |
**update_setting_request** | [**UpdateSettingRequest**](UpdateSettingRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodSetting**](scaleway.baremetal.v1.Setting.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

