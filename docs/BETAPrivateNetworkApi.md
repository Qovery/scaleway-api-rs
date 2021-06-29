# \BETAPrivateNetworkApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_private_network**](BETAPrivateNetworkApi.md#attach_private_network) | **post** /lb/v1/regions/{region}/lbs/{lb_id}/private-networks/{private_network_id}/attach | BETA - Add load balancer on instance private network
[**detach_private_network**](BETAPrivateNetworkApi.md#detach_private_network) | **post** /lb/v1/regions/{region}/lbs/{lb_id}/private-networks/{private_network_id}/detach | BETA - Remove load balancer of private network
[**list_lb_private_networks**](BETAPrivateNetworkApi.md#list_lb_private_networks) | **get** /lb/v1/regions/{region}/lbs/{lb_id}/private-networks | BETA - List attached private network of load balancer



## attach_private_network

> crate::models::ScalewayLbV1PrivateNetwork attach_private_network(region, lb_id, private_network_id, inline_object47)
BETA - Add load balancer on instance private network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**private_network_id** | **String** | Set your instance private network id | [required] |
**inline_object47** | [**InlineObject47**](InlineObject47.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1PrivateNetwork**](scaleway.lb.v1.PrivateNetwork.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_private_network

> detach_private_network(region, private_network_id, lb_id, body)
BETA - Remove load balancer of private network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**private_network_id** | **String** |  | [required] |
**lb_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_lb_private_networks

> crate::models::ScalewayLbV1ListLbPrivateNetworksResponse list_lb_private_networks(region, lb_id, order_by, page_size, page)
BETA - List attached private network of load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** |  | [required] |
**order_by** | Option<[**crate::models::ScalewayLbV1ListPrivateNetworksRequestOrderBy**](.md)> |  |  |
**page_size** | Option<**f32**> | Page size |  |[default to 20]
**page** | Option<**f32**> | Page number |  |[default to 1]

### Return type

[**crate::models::ScalewayLbV1ListLbPrivateNetworksResponse**](scaleway.lb.v1.ListLbPrivateNetworksResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

