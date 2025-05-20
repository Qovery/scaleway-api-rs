# \NodeTypesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_node_types**](NodeTypesApi.md#list_node_types) | **GET** /rdb/v1/regions/{region}/node-types | List available node types



## list_node_types

> models::ScalewayPeriodRdbPeriodV1PeriodListNodeTypesResponse list_node_types(region, include_disabled_types, page, page_size)
List available node types

List all available node types. By default, the node types returned in the list are ordered by creation date in ascending order, though this can be modified via the `order_by` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**include_disabled_types** | **bool** | Defines whether or not to include disabled types. | [required] |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodListNodeTypesResponse**](scaleway.rdb.v1.ListNodeTypesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

