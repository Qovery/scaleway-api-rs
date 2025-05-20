# \PrivateNetworksApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_private_network**](PrivateNetworksApi.md#create_private_network) | **POST** /vpc/v1/zones/{zone}/private-networks | Create a Private Network
[**delete_private_network**](PrivateNetworksApi.md#delete_private_network) | **DELETE** /vpc/v1/zones/{zone}/private-networks/{private_network_id} | Delete a Private Network
[**get_private_network**](PrivateNetworksApi.md#get_private_network) | **GET** /vpc/v1/zones/{zone}/private-networks/{private_network_id} | Get a Private Network
[**list_private_networks**](PrivateNetworksApi.md#list_private_networks) | **GET** /vpc/v1/zones/{zone}/private-networks | List Private Networks
[**update_private_network**](PrivateNetworksApi.md#update_private_network) | **PATCH** /vpc/v1/zones/{zone}/private-networks/{private_network_id} | Update Private Network



## create_private_network

> models::ScalewayPeriodVpcPeriodV1PeriodPrivateNetwork create_private_network(zone, create_private_network_request)
Create a Private Network

Create a new Private Network. Once created, you can attach Scaleway resources in the same Availability Zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_private_network_request** | [**CreatePrivateNetworkRequest**](CreatePrivateNetworkRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodVpcPeriodV1PeriodPrivateNetwork**](scaleway.vpc.v1.PrivateNetwork.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_private_network

> delete_private_network(zone, private_network_id)
Delete a Private Network

Delete an existing Private Network. Note that you must first detach all resources from the network, in order to delete it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**private_network_id** | **String** | Private Network ID. | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_private_network

> models::ScalewayPeriodVpcPeriodV1PeriodPrivateNetwork get_private_network(zone, private_network_id)
Get a Private Network

Retrieve information about an existing Private Network, specified by its Private Network ID. Its full details are returned in the response object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**private_network_id** | **String** | Private Network ID. | [required] |

### Return type

[**models::ScalewayPeriodVpcPeriodV1PeriodPrivateNetwork**](scaleway.vpc.v1.PrivateNetwork.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_private_networks

> models::ScalewayPeriodVpcPeriodV1PeriodListPrivateNetworksResponse list_private_networks(zone, order_by, page, page_size, name, tags, organization_id, project_id, private_network_ids, include_regional)
List Private Networks

List existing Private Networks in a specified Availability Zone. By default, the Private Networks returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**order_by** | Option<**String**> | Sort order of the returned Private Networks. |  |[default to created_at_asc]
**page** | Option<**i32**> | Page number to return, from the paginated results. |  |
**page_size** | Option<**i32**> | Maximum number of Private Networks to return per page. |  |
**name** | Option<**String**> | Name to filter for. Only Private Networks with names containing this string will be returned. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Tags to filter for. Only Private Networks with one or more matching tags will be returned. |  |
**organization_id** | Option<**String**> | Organization ID to filter for. Only Private Networks belonging to this Organization will be returned. |  |
**project_id** | Option<**String**> | Project ID to filter for. Only Private Networks belonging to this Project will be returned. |  |
**private_network_ids** | Option<[**Vec<String>**](String.md)> | Private Network IDs to filter for. Only Private Networks with one of these IDs will be returned. (UUID format) |  |
**include_regional** | Option<**bool**> | Defines whether to include regional Private Networks in the response. |  |

### Return type

[**models::ScalewayPeriodVpcPeriodV1PeriodListPrivateNetworksResponse**](scaleway.vpc.v1.ListPrivateNetworksResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_private_network

> models::ScalewayPeriodVpcPeriodV1PeriodPrivateNetwork update_private_network(zone, private_network_id, update_private_network_request)
Update Private Network

Update parameters (such as name or tags) of an existing Private Network, specified by its Private Network ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**private_network_id** | **String** | Private Network ID. | [required] |
**update_private_network_request** | [**UpdatePrivateNetworkRequest**](UpdatePrivateNetworkRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodVpcPeriodV1PeriodPrivateNetwork**](scaleway.vpc.v1.PrivateNetwork.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

