# \LoadBalancerApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_lb**](LoadBalancerApi.md#create_lb) | **post** /lb/v1/regions/{region}/lbs | Create a load balancer
[**delete_lb**](LoadBalancerApi.md#delete_lb) | **delete** /lb/v1/regions/{region}/lbs/{lb_id} | Delete a load balancer
[**get_lb**](LoadBalancerApi.md#get_lb) | **get** /lb/v1/regions/{region}/lbs/{lb_id} | Get a load balancer
[**list_lb_types**](LoadBalancerApi.md#list_lb_types) | **get** /lb/v1/regions/{region}/lb-types | List all load balancer offer type
[**list_lbs**](LoadBalancerApi.md#list_lbs) | **get** /lb/v1/regions/{region}/lbs | List load balancers
[**migrate_lb**](LoadBalancerApi.md#migrate_lb) | **post** /lb/v1/regions/{region}/lbs/{lb_id}/migrate | Migrate a load balancer
[**update_lb**](LoadBalancerApi.md#update_lb) | **put** /lb/v1/regions/{region}/lbs/{lb_id} | Update a load balancer



## create_lb

> crate::models::ScalewayLbV1Lb create_lb(region, inline_object41)
Create a load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**inline_object41** | [**InlineObject41**](InlineObject41.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Lb**](scaleway.lb.v1.Lb.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lb

> delete_lb(region, lb_id, release_ip)
Delete a load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**release_ip** | Option<**bool**> | Set true if you don't want to keep this IP address |  |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lb

> crate::models::ScalewayLbV1Lb get_lb(region, lb_id)
Get a load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |

### Return type

[**crate::models::ScalewayLbV1Lb**](scaleway.lb.v1.Lb.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_lb_types

> crate::models::ScalewayLbV1ListLbTypesResponse list_lb_types(region, page, page_size)
List all load balancer offer type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | **f32** | Page number | [required] |[default to 1]
**page_size** | Option<**f32**> | The number of items to return |  |[default to 20]

### Return type

[**crate::models::ScalewayLbV1ListLbTypesResponse**](scaleway.lb.v1.ListLbTypesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_lbs

> crate::models::ScalewayLbV1ListLbsResponse list_lbs(region, name, order_by, page_size, page, organization_id, project_id)
List load balancers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**name** | Option<**String**> | Use this to search by name |  |
**order_by** | Option<[**crate::models::ScalewayLbV1ListLbsRequestOrderBy**](.md)> |  |  |
**page_size** | Option<**f32**> | Page size |  |[default to 20]
**page** | Option<**f32**> | Page number |  |[default to 1]
**organization_id** | Option<**String**> | Filter LBs by organization ID |  |
**project_id** | Option<**String**> | Filter LBs by project ID |  |

### Return type

[**crate::models::ScalewayLbV1ListLbsResponse**](scaleway.lb.v1.ListLbsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrate_lb

> crate::models::ScalewayLbV1Lb migrate_lb(region, lb_id, inline_object46)
Migrate a load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**inline_object46** | [**InlineObject46**](InlineObject46.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Lb**](scaleway.lb.v1.Lb.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_lb

> crate::models::ScalewayLbV1Lb update_lb(region, lb_id, inline_object42)
Update a load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**inline_object42** | [**InlineObject42**](InlineObject42.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Lb**](scaleway.lb.v1.Lb.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

