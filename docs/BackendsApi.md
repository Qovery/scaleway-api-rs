# \BackendsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_backend_servers**](BackendsApi.md#add_backend_servers) | **post** /lb/v1/regions/{region}/backends/{backend_id}/servers | Add a set of servers in a given backend
[**create_backend**](BackendsApi.md#create_backend) | **post** /lb/v1/regions/{region}/lbs/{lb_id}/backends | Create a backend in a given load balancer
[**delete_backend**](BackendsApi.md#delete_backend) | **delete** /lb/v1/regions/{region}/backends/{backend_id} | Delete a backend in a given load balancer
[**get_backend**](BackendsApi.md#get_backend) | **get** /lb/v1/regions/{region}/backends/{backend_id} | Get a backend in a given load balancer
[**list_backends**](BackendsApi.md#list_backends) | **get** /lb/v1/regions/{region}/lbs/{lb_id}/backends | List backends in a given load balancer
[**remove_backend_servers**](BackendsApi.md#remove_backend_servers) | **delete** /lb/v1/regions/{region}/backends/{backend_id}/servers | Remove a set of servers for a given backend
[**set_backend_servers**](BackendsApi.md#set_backend_servers) | **put** /lb/v1/regions/{region}/backends/{backend_id}/servers | Define all servers in a given backend
[**update_backend**](BackendsApi.md#update_backend) | **put** /lb/v1/regions/{region}/backends/{backend_id} | Update a backend in a given load balancer
[**update_health_check**](BackendsApi.md#update_health_check) | **put** /lb/v1/regions/{region}/backends/{backend_id}/healthcheck | Update an healthcheck for a given backend



## add_backend_servers

> crate::models::ScalewayLbV1Backend add_backend_servers(region, backend_id, inline_object33)
Add a set of servers in a given backend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**backend_id** | **String** | Backend ID | [required] |
**inline_object33** | [**InlineObject33**](InlineObject33.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Backend**](scaleway.lb.v1.Backend.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_backend

> crate::models::ScalewayLbV1Backend create_backend(region, lb_id, inline_object43)
Create a backend in a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**inline_object43** | [**InlineObject43**](InlineObject43.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Backend**](scaleway.lb.v1.Backend.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_backend

> delete_backend(region, backend_id)
Delete a backend in a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**backend_id** | **String** | ID of the backend to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backend

> crate::models::ScalewayLbV1Backend get_backend(region, backend_id)
Get a backend in a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**backend_id** | **String** | Backend ID | [required] |

### Return type

[**crate::models::ScalewayLbV1Backend**](scaleway.lb.v1.Backend.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_backends

> crate::models::ScalewayLbV1ListBackendsResponse list_backends(region, lb_id, page, name, order_by, page_size)
List backends in a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**page** | **f32** | Page number | [required] |[default to 1]
**name** | Option<**String**> | Use this to search by name |  |
**order_by** | Option<**String**> | Choose order of response |  |[default to created_at_asc]
**page_size** | Option<**f32**> | The number of items to returns |  |[default to 20]

### Return type

[**crate::models::ScalewayLbV1ListBackendsResponse**](scaleway.lb.v1.ListBackendsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_backend_servers

> crate::models::ScalewayLbV1Backend remove_backend_servers(region, backend_id, inline_object34)
Remove a set of servers for a given backend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**backend_id** | **String** | Backend ID | [required] |
**inline_object34** | [**InlineObject34**](InlineObject34.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Backend**](scaleway.lb.v1.Backend.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_backend_servers

> crate::models::ScalewayLbV1Backend set_backend_servers(region, backend_id, inline_object32)
Define all servers in a given backend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**backend_id** | **String** | Backend ID | [required] |
**inline_object32** | [**InlineObject32**](InlineObject32.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Backend**](scaleway.lb.v1.Backend.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_backend

> crate::models::ScalewayLbV1Backend update_backend(region, backend_id, inline_object30)
Update a backend in a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**backend_id** | **String** |  | [required] |
**inline_object30** | [**InlineObject30**](InlineObject30.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Backend**](scaleway.lb.v1.Backend.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_health_check

> crate::models::ScalewayLbV1HealthCheck update_health_check(region, backend_id, inline_object31)
Update an healthcheck for a given backend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**backend_id** | **String** | Backend ID | [required] |
**inline_object31** | [**InlineObject31**](InlineObject31.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1HealthCheck**](scaleway.lb.v1.HealthCheck.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

