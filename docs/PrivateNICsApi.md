# \PrivateNICsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_private_nic**](PrivateNICsApi.md#create_private_nic) | **post** /instance/v1/zones/{zone}/servers/{server_id}/private_nics | Create a private NIC connecting a server to a private network
[**delete_private_nic**](PrivateNICsApi.md#delete_private_nic) | **delete** /instance/v1/zones/{zone}/servers/{server_id}/private_nics/{private_nic_id} | Delete a private NIC
[**get_private_nic**](PrivateNICsApi.md#get_private_nic) | **get** /instance/v1/zones/{zone}/servers/{server_id}/private_nics/{private_nic_id} | Get a private NIC
[**list_private_nics**](PrivateNICsApi.md#list_private_nics) | **get** /instance/v1/zones/{zone}/servers/{server_id}/private_nics | List all private NICs



## create_private_nic

> crate::models::ScalewayInstanceV1CreatePrivateNicResponse create_private_nic(zone, server_id, inline_object16)
Create a private NIC connecting a server to a private network

Create a private NIC connecting a server to a private network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** |  | [required] |
**inline_object16** | [**InlineObject16**](InlineObject16.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1CreatePrivateNicResponse**](scaleway.instance.v1.CreatePrivateNICResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_private_nic

> delete_private_nic(zone, server_id, private_nic_id)
Delete a private NIC

Delete a private NIC.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** |  | [required] |
**private_nic_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_private_nic

> crate::models::ScalewayInstanceV1GetPrivateNicResponse get_private_nic(zone, server_id, private_nic_id)
Get a private NIC

Get private NIC properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** |  | [required] |
**private_nic_id** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1GetPrivateNicResponse**](scaleway.instance.v1.GetPrivateNICResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_private_nics

> crate::models::ScalewayInstanceV1ListPrivateNicsResponse list_private_nics(zone, server_id)
List all private NICs

List all private NICs of a given server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1ListPrivateNicsResponse**](scaleway.instance.v1.ListPrivateNICsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

