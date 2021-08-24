# \NodeTypesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_node_types**](NodeTypesApi.md#list_node_types) | **GET** /rdb/v1/regions/{region}/node-types | List available node types



## list_node_types

> crate::models::ScalewayRdbV1ListNodeTypesResponse list_node_types(region, include_disabled_types, page, page_size)
List available node types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**include_disabled_types** | Option<**bool**> | Whether or not to include disabled types |  |
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayRdbV1ListNodeTypesResponse**](scaleway.rdb.v1.ListNodeTypesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

