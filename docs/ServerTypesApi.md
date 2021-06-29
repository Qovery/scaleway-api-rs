# \ServerTypesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_server_type**](ServerTypesApi.md#get_server_type) | **get** /apple-silicon/v1alpha1/zones/{zone}/server-type/{server_type} | Get a server type
[**get_server_types_availability**](ServerTypesApi.md#get_server_types_availability) | **get** /instance/v1/zones/{zone}/products/servers/availability | Get availability
[**list_server_types**](ServerTypesApi.md#list_server_types) | **get** /apple-silicon/v1alpha1/zones/{zone}/server-types | List server types
[**list_servers_types**](ServerTypesApi.md#list_servers_types) | **get** /instance/v1/zones/{zone}/products/servers | List server types



## get_server_type

> crate::models::ScalewayAppleSiliconV1alpha1ServerType get_server_type(zone, server_type)
Get a server type

Get a server technical details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |
**server_type** | **String** | Server type identifier | [required] |

### Return type

[**crate::models::ScalewayAppleSiliconV1alpha1ServerType**](scaleway.apple_silicon.v1alpha1.ServerType.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_types_availability

> crate::models::ScalewayInstanceV1GetServerTypesAvailabilityResponse get_server_types_availability(zone, per_page, page)
Get availability

Get availibility for all server types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**per_page** | Option<**f32**> |  |  |
**page** | Option<**f32**> | Page number |  |[default to 1]

### Return type

[**crate::models::ScalewayInstanceV1GetServerTypesAvailabilityResponse**](scaleway.instance.v1.GetServerTypesAvailabilityResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_server_types

> crate::models::ScalewayAppleSiliconV1alpha1ListServerTypesResponse list_server_types(zone)
List server types

List all server types technical details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |

### Return type

[**crate::models::ScalewayAppleSiliconV1alpha1ListServerTypesResponse**](scaleway.apple_silicon.v1alpha1.ListServerTypesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_servers_types

> crate::models::ScalewayInstanceV1ListServersTypesResponse list_servers_types(zone, per_page, page)
List server types

Get server types technical details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**per_page** | Option<**f32**> |  |  |
**page** | Option<**f32**> | Page number |  |[default to 1]

### Return type

[**crate::models::ScalewayInstanceV1ListServersTypesResponse**](scaleway.instance.v1.ListServersTypesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

