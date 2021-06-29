# \VolumeTypesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_volumes_types**](VolumeTypesApi.md#list_volumes_types) | **get** /instance/v1/zones/{zone}/products/volumes | List volumes types



## list_volumes_types

> crate::models::ScalewayInstanceV1ListVolumesTypesResponse list_volumes_types(zone, per_page, page)
List volumes types

Get volumes technical details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**per_page** | Option<**f32**> |  |  |
**page** | Option<**f32**> | Page number |  |[default to 1]

### Return type

[**crate::models::ScalewayInstanceV1ListVolumesTypesResponse**](scaleway.instance.v1.ListVolumesTypesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

