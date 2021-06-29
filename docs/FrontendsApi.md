# \FrontendsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_frontend**](FrontendsApi.md#create_frontend) | **post** /lb/v1/regions/{region}/lbs/{lb_id}/frontends | Create a frontend in a given load balancer
[**delete_frontend**](FrontendsApi.md#delete_frontend) | **delete** /lb/v1/regions/{region}/frontends/{frontend_id} | Delete a frontend
[**get_frontend**](FrontendsApi.md#get_frontend) | **get** /lb/v1/regions/{region}/frontends/{frontend_id} | Get a frontend
[**list_frontends**](FrontendsApi.md#list_frontends) | **get** /lb/v1/regions/{region}/lbs/{lb_id}/frontends | List frontends in a given load balancer
[**update_frontend**](FrontendsApi.md#update_frontend) | **put** /lb/v1/regions/{region}/frontends/{frontend_id} | Update a frontend



## create_frontend

> crate::models::ScalewayLbV1Frontend create_frontend(region, lb_id, inline_object45)
Create a frontend in a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**inline_object45** | [**InlineObject45**](InlineObject45.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Frontend**](scaleway.lb.v1.Frontend.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_frontend

> delete_frontend(region, frontend_id)
Delete a frontend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**frontend_id** | **String** | Frontend ID to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_frontend

> crate::models::ScalewayLbV1Frontend get_frontend(region, frontend_id)
Get a frontend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**frontend_id** | **String** | Frontend ID | [required] |

### Return type

[**crate::models::ScalewayLbV1Frontend**](scaleway.lb.v1.Frontend.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_frontends

> crate::models::ScalewayLbV1ListFrontendsResponse list_frontends(region, lb_id, page, name, order_by, page_size)
List frontends in a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**page** | **f32** | Page number | [required] |[default to 1]
**name** | Option<**String**> | Use this to search by name |  |
**order_by** | Option<**String**> | Response order |  |[default to created_at_asc]
**page_size** | Option<**f32**> | The number of items to returns |  |[default to 20]

### Return type

[**crate::models::ScalewayLbV1ListFrontendsResponse**](scaleway.lb.v1.ListFrontendsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_frontend

> crate::models::ScalewayLbV1Frontend update_frontend(region, frontend_id, inline_object36)
Update a frontend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**frontend_id** | **String** | Frontend ID | [required] |
**inline_object36** | [**InlineObject36**](InlineObject36.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Frontend**](scaleway.lb.v1.Frontend.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

