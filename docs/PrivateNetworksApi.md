# \PrivateNetworksApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_private_network**](PrivateNetworksApi.md#create_private_network) | **POST** /vpc/v1/zones/{zone}/private-networks | Create a private network
[**delete_private_network**](PrivateNetworksApi.md#delete_private_network) | **DELETE** /vpc/v1/zones/{zone}/private-networks/{private_network_id} | Delete a private network
[**get_private_network**](PrivateNetworksApi.md#get_private_network) | **GET** /vpc/v1/zones/{zone}/private-networks/{private_network_id} | Get a private network
[**list_private_networks**](PrivateNetworksApi.md#list_private_networks) | **GET** /vpc/v1/zones/{zone}/private-networks | List private networks
[**update_private_network**](PrivateNetworksApi.md#update_private_network) | **PATCH** /vpc/v1/zones/{zone}/private-networks/{private_network_id} | Update private network



## create_private_network

> crate::models::ScalewayVpcV1PrivateNetwork create_private_network(zone, inline_object37)
Create a private network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object37** | [**InlineObject37**](InlineObject37.md) |  | [required] |

### Return type

[**crate::models::ScalewayVpcV1PrivateNetwork**](scaleway.vpc.v1.PrivateNetwork.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_private_network

> delete_private_network(zone, private_network_id)
Delete a private network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**private_network_id** | **String** | The private network ID | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_private_network

> crate::models::ScalewayVpcV1PrivateNetwork get_private_network(zone, private_network_id)
Get a private network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**private_network_id** | **String** | The private network id | [required] |

### Return type

[**crate::models::ScalewayVpcV1PrivateNetwork**](scaleway.vpc.v1.PrivateNetwork.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_private_networks

> crate::models::ScalewayVpcV1ListPrivateNetworksResponse list_private_networks(zone, order_by, page, page_size, name, tags, organization_id, project_id)
List private networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**order_by** | Option<**String**> | The sort order of the returned private networks |  |[default to created_at_asc]
**page** | Option<**f32**> | The page number for the returned private networks |  |[default to 1]
**page_size** | Option<**f32**> | The maximum number of private networks per page |  |[default to 20]
**name** | Option<**String**> | Filter private networks with names containing this string |  |
**tags** | Option<[**Vec<String>**](String.md)> | Filter private networks with one or more matching tags |  |
**organization_id** | Option<**String**> | The organization ID on which to filter the returned private networks |  |
**project_id** | Option<**String**> | The project ID on which to filter the returned private networks |  |

### Return type

[**crate::models::ScalewayVpcV1ListPrivateNetworksResponse**](scaleway.vpc.v1.ListPrivateNetworksResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_private_network

> crate::models::ScalewayVpcV1PrivateNetwork update_private_network(zone, private_network_id, inline_object38)
Update private network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**private_network_id** | **String** | The private network ID | [required] |
**inline_object38** | [**InlineObject38**](InlineObject38.md) |  | [required] |

### Return type

[**crate::models::ScalewayVpcV1PrivateNetwork**](scaleway.vpc.v1.PrivateNetwork.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

