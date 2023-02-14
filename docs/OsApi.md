# \OsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_os**](OsApi.md#get_os) | **GET** /baremetal/v1/zones/{zone}/os/{os_id} | Get an OS with a given ID
[**list_os**](OsApi.md#list_os) | **GET** /baremetal/v1/zones/{zone}/os | List all available OS that can be install on an elastic metal server



## get_os

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodOs get_os(zone, os_id)
Get an OS with a given ID

Return specific OS for the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**os_id** | **String** | ID of the OS | [required] |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodOs**](scaleway.baremetal.v1.OS.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_os

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodListOsResponse list_os(zone, page, page_size, offer_id)
List all available OS that can be install on an elastic metal server

List all available OS that can be install on an elastic metal server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**page** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Number of OS per page |  |[default to 20]
**offer_id** | Option<**String**> | Filter OS by offer ID |  |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodListOsResponse**](scaleway.baremetal.v1.ListOSResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

