# \OptionsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_option**](OptionsApi.md#get_option) | **GET** /baremetal/v1/zones/{zone}/options/{option_id} | Get option
[**list_options**](OptionsApi.md#list_options) | **GET** /baremetal/v1/zones/{zone}/options | List options
[**list_settings**](OptionsApi.md#list_settings) | **GET** /baremetal/v1/zones/{zone}/settings | List all settings
[**update_setting**](OptionsApi.md#update_setting) | **PATCH** /baremetal/v1/zones/{zone}/settings/{setting_id} | Update setting



## get_option

> crate::models::ScalewayBaremetalV1Option get_option(zone, option_id)
Get option

Return specific option for the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**option_id** | **String** | ID of the option | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Option**](scaleway.baremetal.v1.Option.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_options

> crate::models::ScalewayBaremetalV1ListOptionsResponse list_options(zone, page, page_size, offer_id, name)
List options

List all options matching with filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**page** | Option<**i64**> | Page number |  |[default to 1]
**page_size** | Option<**i64**> | Number of options per page |  |[default to 20]
**offer_id** | Option<**String**> | Filter options by offer_id |  |
**name** | Option<**String**> | Filter options by name |  |

### Return type

[**crate::models::ScalewayBaremetalV1ListOptionsResponse**](scaleway.baremetal.v1.ListOptionsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_settings

> crate::models::ScalewayBaremetalV1ListSettingsResponse list_settings(zone, project_id, page, page_size, order_by)
List all settings

Return all settings for a project ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**project_id** | Option<**String**> | ID of the project (UUID format) | [required] |
**page** | Option<**i64**> | Page number |  |[default to 1]
**page_size** | Option<**i64**> | Set the maximum list size |  |[default to 20]
**order_by** | Option<**String**> | Order the response |  |[default to created_at_asc]

### Return type

[**crate::models::ScalewayBaremetalV1ListSettingsResponse**](scaleway.baremetal.v1.ListSettingsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_setting

> crate::models::ScalewayBaremetalV1Setting update_setting(zone, setting_id, update_setting_request)
Update setting

Update a setting for a project ID (enable or disable).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**setting_id** | **String** | ID of the setting | [required] |
**update_setting_request** | [**UpdateSettingRequest**](UpdateSettingRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Setting**](scaleway.baremetal.v1.Setting.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

