# \ServerTypesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_server_types_availability**](ServerTypesApi.md#get_server_types_availability) | **GET** /instance/v1/zones/{zone}/products/servers/availability | Get availability
[**list_servers_types**](ServerTypesApi.md#list_servers_types) | **GET** /instance/v1/zones/{zone}/products/servers | List server types



## get_server_types_availability

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetServerTypesAvailabilityResponse get_server_types_availability(zone, per_page, page)
Get availability

Get availability for all server types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**per_page** | Option<**i32**> |  |  |
**page** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetServerTypesAvailabilityResponse**](scaleway.instance.v1.GetServerTypesAvailabilityResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_servers_types

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListServersTypesResponse list_servers_types(zone, per_page, page)
List server types

Get server types technical details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**per_page** | Option<**i32**> |  |  |
**page** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListServersTypesResponse**](scaleway.instance.v1.ListServersTypesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

