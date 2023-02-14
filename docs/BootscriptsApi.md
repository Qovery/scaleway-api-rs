# \BootscriptsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bootscript**](BootscriptsApi.md#get_bootscript) | **GET** /instance/v1/zones/{zone}/bootscripts/{bootscript_id} | Get bootscripts
[**list_bootscripts**](BootscriptsApi.md#list_bootscripts) | **GET** /instance/v1/zones/{zone}/bootscripts | List bootscripts



## get_bootscript

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetBootscriptResponse get_bootscript(zone, bootscript_id)
Get bootscripts

Get details of a bootscript with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**bootscript_id** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetBootscriptResponse**](scaleway.instance.v1.GetBootscriptResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_bootscripts

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListBootscriptsResponse list_bootscripts(zone, arch, title, default, public, per_page, page)
List bootscripts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**arch** | Option<**String**> |  |  |
**title** | Option<**String**> |  |  |
**default** | Option<**bool**> |  |  |
**public** | Option<**bool**> |  |  |
**per_page** | Option<**i32**> |  |  |
**page** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListBootscriptsResponse**](scaleway.instance.v1.ListBootscriptsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

