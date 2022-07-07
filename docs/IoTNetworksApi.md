# \IoTNetworksApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_network**](IoTNetworksApi.md#create_network) | **POST** /iot/v1/regions/{region}/networks | Create a new Network
[**delete_network**](IoTNetworksApi.md#delete_network) | **DELETE** /iot/v1/regions/{region}/networks/{network_id} | Delete a Network
[**get_network**](IoTNetworksApi.md#get_network) | **GET** /iot/v1/regions/{region}/networks/{network_id} | Retrieve a specific Network
[**list_networks**](IoTNetworksApi.md#list_networks) | **GET** /iot/v1/regions/{region}/networks | List the Networks



## create_network

> crate::models::ScalewayIotV1CreateNetworkResponse create_network(region, create_network_request)
Create a new Network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**create_network_request** | [**CreateNetworkRequest**](CreateNetworkRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1CreateNetworkResponse**](scaleway.iot.v1.CreateNetworkResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_network

> delete_network(region, network_id)
Delete a Network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**network_id** | **String** | Network ID | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network

> crate::models::ScalewayIotV1Network get_network(region, network_id)
Retrieve a specific Network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**network_id** | **String** | Network ID | [required] |

### Return type

[**crate::models::ScalewayIotV1Network**](scaleway.iot.v1.Network.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_networks

> crate::models::ScalewayIotV1ListNetworksResponse list_networks(region, page, page_size, order_by, name, hub_id, topic_prefix)
List the Networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | Option<**i64**> | Page number |  |[default to 1]
**page_size** | Option<**i64**> | Page size. The maximum value is 100 |  |[default to 20]
**order_by** | Option<**String**> | Ordering of requested routes |  |[default to name_asc]
**name** | Option<**String**> | Filter on Network name |  |
**hub_id** | Option<**String**> | Filter on the hub |  |
**topic_prefix** | Option<**String**> | Filter on the topic prefix |  |

### Return type

[**crate::models::ScalewayIotV1ListNetworksResponse**](scaleway.iot.v1.ListNetworksResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

