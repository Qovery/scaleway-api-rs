# \RouteApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_route**](RouteApi.md#create_route) | **post** /lb/v1/regions/{region}/routes | Create a backend redirection
[**delete_route**](RouteApi.md#delete_route) | **delete** /lb/v1/regions/{region}/routes/{route_id} | Delete a backend redirection
[**get_route**](RouteApi.md#get_route) | **get** /lb/v1/regions/{region}/routes/{route_id} | Get single backend redirection
[**list_routes**](RouteApi.md#list_routes) | **get** /lb/v1/regions/{region}/routes | List all backend redirections
[**update_route**](RouteApi.md#update_route) | **put** /lb/v1/regions/{region}/routes/{route_id} | Edit a backend redirection



## create_route

> crate::models::ScalewayLbV1Route create_route(region, inline_object48)
Create a backend redirection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**inline_object48** | [**InlineObject48**](InlineObject48.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Route**](scaleway.lb.v1.Route.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_route

> delete_route(region, route_id)
Delete a backend redirection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**route_id** | **String** | Route id to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_route

> crate::models::ScalewayLbV1Route get_route(region, route_id)
Get single backend redirection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**route_id** | **String** | Id of route to get | [required] |

### Return type

[**crate::models::ScalewayLbV1Route**](scaleway.lb.v1.Route.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_routes

> crate::models::ScalewayLbV1ListRoutesResponse list_routes(region, order_by, page_size, page, frontend_id)
List all backend redirections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**order_by** | Option<[**crate::models::ScalewayLbV1ListRoutesRequestOrderBy**](.md)> |  |  |
**page_size** | Option<**f32**> | Page size |  |[default to 20]
**page** | Option<**f32**> | Page number |  |[default to 1]
**frontend_id** | Option<**String**> |  |  |

### Return type

[**crate::models::ScalewayLbV1ListRoutesResponse**](scaleway.lb.v1.ListRoutesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_route

> crate::models::ScalewayLbV1Route update_route(region, route_id, inline_object49)
Edit a backend redirection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**route_id** | **String** | Route id to update | [required] |
**inline_object49** | [**InlineObject49**](InlineObject49.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Route**](scaleway.lb.v1.Route.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

