# \ClusterTypesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_cluster_types**](ClusterTypesApi.md#list_cluster_types) | **GET** /k8s/v1/regions/{region}/cluster-types | List cluster types



## list_cluster_types

> models::ScalewayPeriodK8sPeriodV1PeriodListClusterTypesResponse list_cluster_types(region, page, page_size)
List cluster types

List available cluster types and their technical details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | Option<**i32**> | Page number, from the paginated results, to return for cluster-types. |  |
**page_size** | Option<**i32**> | Maximum number of clusters per page. |  |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodListClusterTypesResponse**](scaleway.k8s.v1.ListClusterTypesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

