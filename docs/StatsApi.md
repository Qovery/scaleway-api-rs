# \StatsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lb_stats**](StatsApi.md#get_lb_stats) | **get** /lb/v1/regions/{region}/lbs/{lb_id}/stats | Get usage statistics of a given load balancer



## get_lb_stats

> crate::models::ScalewayLbV1LbStats get_lb_stats(region, lb_id)
Get usage statistics of a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |

### Return type

[**crate::models::ScalewayLbV1LbStats**](scaleway.lb.v1.LbStats.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

