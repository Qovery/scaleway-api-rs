# \EndpointsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_endpoint**](EndpointsApi.md#create_endpoint) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/endpoints | Create a new instance endpoint
[**delete_endpoint**](EndpointsApi.md#delete_endpoint) | **DELETE** /rdb/v1/regions/{region}/endpoints/{endpoint_id} | Delete an instance endpoint
[**get_endpoint**](EndpointsApi.md#get_endpoint) | **GET** /rdb/v1/regions/{region}/endpoints/{endpoint_id} | Get an instance endpoint



## create_endpoint

> crate::models::ScalewayRdbV1Endpoint create_endpoint(region, instance_id, create_endpoint_request)
Create a new instance endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to add endpoint to | [required] |
**create_endpoint_request** | [**CreateEndpointRequest**](CreateEndpointRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Endpoint**](scaleway.rdb.v1.Endpoint.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_endpoint

> delete_endpoint(region, endpoint_id)
Delete an instance endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**endpoint_id** | **String** | UUID of the endpoint you want to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_endpoint

> crate::models::ScalewayRdbV1Endpoint get_endpoint(region, endpoint_id)
Get an instance endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**endpoint_id** | **String** | UUID of the endpoint you want to get | [required] |

### Return type

[**crate::models::ScalewayRdbV1Endpoint**](scaleway.rdb.v1.Endpoint.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

