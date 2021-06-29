# \OsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_os1**](OsApi.md#get_os1) | **get** /apple-silicon/v1alpha1/zones/{zone}/os/{os_id} | Get an Operating System (OS)
[**list_os1**](OsApi.md#list_os1) | **get** /apple-silicon/v1alpha1/zones/{zone}/os | List all Operating System (OS)



## get_os1

> crate::models::ScalewayAppleSiliconV1alpha1Os get_os1(zone, os_id)
Get an Operating System (OS)

Get an Operating System (OS).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |
**os_id** | **String** | UUID of the OS you want to get | [required] |

### Return type

[**crate::models::ScalewayAppleSiliconV1alpha1Os**](scaleway.apple_silicon.v1alpha1.OS.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_os1

> crate::models::ScalewayAppleSiliconV1alpha1ListOsResponse list_os1(zone, page, page_size, server_type, name)
List all Operating System (OS)

List all Operating System (OS).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |
**page** | Option<**f32**> | A positive integer to choose the page to return |  |[default to 1]
**page_size** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**server_type** | Option<**String**> | List of compatible server type |  |
**name** | Option<**String**> | Filter os by name (for eg. \"11.1\" will return \"11.1.2\" and \"11.1\" but not \"12\") |  |

### Return type

[**crate::models::ScalewayAppleSiliconV1alpha1ListOsResponse**](scaleway.apple_silicon.v1alpha1.ListOSResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

