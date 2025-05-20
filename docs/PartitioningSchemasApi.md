# \PartitioningSchemasApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_default_partitioning_schema**](PartitioningSchemasApi.md#get_default_partitioning_schema) | **GET** /baremetal/v1/zones/{zone}/partitioning-schemas/default | Get default partitioning schema
[**validate_partitioning_schema**](PartitioningSchemasApi.md#validate_partitioning_schema) | **POST** /baremetal/v1/zones/{zone}/partitioning-schemas/validate | Validate client partitioning schema



## get_default_partitioning_schema

> models::ScalewayPeriodBaremetalPeriodV1PeriodSchema get_default_partitioning_schema(zone, offer_id, os_id)
Get default partitioning schema

Get the default partitioning schema for the given offer ID and OS ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**offer_id** | **String** | ID of the offer. | [required] |
**os_id** | **String** | ID of the OS. | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodSchema**](scaleway.baremetal.v1.Schema.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_partitioning_schema

> validate_partitioning_schema(zone, validate_partitioning_schema_request)
Validate client partitioning schema

Validate the incoming partitioning schema from a user before installing the server. Return default ErrorCode if invalid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**validate_partitioning_schema_request** | [**ValidatePartitioningSchemaRequest**](ValidatePartitioningSchemaRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

