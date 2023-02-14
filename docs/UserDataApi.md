# \UserDataApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_server_user_data**](UserDataApi.md#delete_server_user_data) | **DELETE** /instance/v1/zones/{zone}/servers/{server_id}/user_data/{key} | Delete user data
[**get_server_user_data**](UserDataApi.md#get_server_user_data) | **GET** /instance/v1/zones/{zone}/servers/{server_id}/user_data/{key} | Get user data
[**list_server_user_data**](UserDataApi.md#list_server_user_data) | **GET** /instance/v1/zones/{zone}/servers/{server_id}/user_data | List user data
[**set_server_user_data**](UserDataApi.md#set_server_user_data) | **PATCH** /instance/v1/zones/{zone}/servers/{server_id}/user_data/{key} | Add/Set user data



## delete_server_user_data

> delete_server_user_data(zone, server_id, key)
Delete user data

Delete the given key from a server user data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | UUID of the server | [required] |
**key** | **String** | Key of the user data to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_user_data

> crate::models::ScalewayPeriodStdPeriodFile get_server_user_data(zone, server_id, key)
Get user data

Get the content of a user data with the given key on a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | UUID of the server | [required] |
**key** | **String** | Key of the user data to get | [required] |

### Return type

[**crate::models::ScalewayPeriodStdPeriodFile**](scaleway.std.File.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_server_user_data

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListServerUserDataResponse list_server_user_data(zone, server_id)
List user data

List all user data keys registered on a given server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | UUID of the server | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListServerUserDataResponse**](scaleway.instance.v1.ListServerUserDataResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_server_user_data

> set_server_user_data(zone, server_id, key, body)
Add/Set user data

Add or update a user data with the given key on a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | UUID of the server | [required] |
**key** | **String** | Key of the user data to set | [required] |
**body** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

