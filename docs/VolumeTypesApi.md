# \VolumeTypesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_volumes_types**](VolumeTypesApi.md#list_volumes_types) | **GET** /instance/v1/zones/{zone}/products/volumes | List volumes types



## list_volumes_types

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListVolumesTypesResponse list_volumes_types(zone, per_page, page)
List volumes types

Get volumes technical details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**per_page** | Option<**i32**> |  |  |
**page** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListVolumesTypesResponse**](scaleway.instance.v1.ListVolumesTypesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

