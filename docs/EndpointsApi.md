# \EndpointsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_endpoint**](EndpointsApi.md#create_endpoint) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/endpoints | Create a new Database Instance endpoint
[**delete_endpoint**](EndpointsApi.md#delete_endpoint) | **DELETE** /rdb/v1/regions/{region}/endpoints/{endpoint_id} | Delete a Database Instance endpoint
[**get_endpoint**](EndpointsApi.md#get_endpoint) | **GET** /rdb/v1/regions/{region}/endpoints/{endpoint_id} | Get a Database Instance endpoint
[**migrate_endpoint**](EndpointsApi.md#migrate_endpoint) | **POST** /rdb/v1/regions/{region}/endpoints/{endpoint_id}/migrate | Migrate an existing instance endpoint to another instance



## create_endpoint

> models::ScalewayPeriodRdbPeriodV1PeriodEndpoint create_endpoint(region, instance_id, create_endpoint_request)
Create a new Database Instance endpoint

Create a new endpoint for a Database Instance. You can add `load_balancer` and `private_network` specifications to the body of the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you to which you want to add an endpoint. | [required] |
**create_endpoint_request** | [**CreateEndpointRequest**](CreateEndpointRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodEndpoint**](scaleway.rdb.v1.Endpoint.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_endpoint

> delete_endpoint(region, endpoint_id)
Delete a Database Instance endpoint

Delete the endpoint of a Database Instance. You must specify the `region` and `endpoint_id` parameters of the endpoint you want to delete. Note that might need to update any environment configurations that point to the deleted endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**endpoint_id** | **String** | UUID of the endpoint you want to delete. This endpoint can also be used to delete a Read Replica endpoint. | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_endpoint

> models::ScalewayPeriodRdbPeriodV1PeriodEndpoint get_endpoint(region, endpoint_id)
Get a Database Instance endpoint

Retrieve information about a Database Instance endpoint. Full details about the endpoint, like `ip`, `port`, `private_network` and `load_balancer` specifications are returned in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**endpoint_id** | **String** | UUID of the endpoint you want to get. | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodEndpoint**](scaleway.rdb.v1.Endpoint.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrate_endpoint

> models::ScalewayPeriodRdbPeriodV1PeriodEndpoint migrate_endpoint(region, endpoint_id, migrate_endpoint_request)
Migrate an existing instance endpoint to another instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**endpoint_id** | **String** | UUID of the endpoint you want to migrate. | [required] |
**migrate_endpoint_request** | [**MigrateEndpointRequest**](MigrateEndpointRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodEndpoint**](scaleway.rdb.v1.Endpoint.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

