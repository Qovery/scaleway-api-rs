# \IoTHubsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_hub**](IoTHubsApi.md#create_hub) | **POST** /iot/v1/regions/{region}/hubs | Create a hub
[**delete_hub**](IoTHubsApi.md#delete_hub) | **DELETE** /iot/v1/regions/{region}/hubs/{hub_id} | Delete a hub
[**disable_hub**](IoTHubsApi.md#disable_hub) | **POST** /iot/v1/regions/{region}/hubs/{hub_id}/disable | Disable a hub
[**enable_hub**](IoTHubsApi.md#enable_hub) | **POST** /iot/v1/regions/{region}/hubs/{hub_id}/enable | Enable a hub
[**get_hub**](IoTHubsApi.md#get_hub) | **GET** /iot/v1/regions/{region}/hubs/{hub_id} | Get a hub
[**get_hub_ca**](IoTHubsApi.md#get_hub_ca) | **GET** /iot/v1/regions/{region}/hubs/{hub_id}/ca | Get the certificate authority of a hub
[**get_hub_metrics**](IoTHubsApi.md#get_hub_metrics) | **GET** /iot/v1/regions/{region}/hubs/{hub_id}/metrics | Get a hub's metrics
[**list_hubs**](IoTHubsApi.md#list_hubs) | **GET** /iot/v1/regions/{region}/hubs | List hubs
[**set_hub_ca**](IoTHubsApi.md#set_hub_ca) | **POST** /iot/v1/regions/{region}/hubs/{hub_id}/ca | Set the certificate authority of a hub
[**update_hub**](IoTHubsApi.md#update_hub) | **PATCH** /iot/v1/regions/{region}/hubs/{hub_id} | Update a hub



## create_hub

> crate::models::ScalewayIotV1Hub create_hub(region, create_hub_request)
Create a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**create_hub_request** | [**CreateHubRequest**](CreateHubRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Hub**](scaleway.iot.v1.Hub.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_hub

> delete_hub(region, hub_id, delete_devices)
Delete a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**hub_id** | **String** | Hub ID | [required] |
**delete_devices** | Option<**bool**> | Force deletion of devices added to this hub instead of rejecting operation |  |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_hub

> crate::models::ScalewayIotV1Hub disable_hub(region, hub_id, body)
Disable a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**hub_id** | **String** | Hub ID | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Hub**](scaleway.iot.v1.Hub.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_hub

> crate::models::ScalewayIotV1Hub enable_hub(region, hub_id, body)
Enable a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**hub_id** | **String** | Hub ID | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Hub**](scaleway.iot.v1.Hub.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hub

> crate::models::ScalewayIotV1Hub get_hub(region, hub_id)
Get a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**hub_id** | **String** | Hub ID | [required] |

### Return type

[**crate::models::ScalewayIotV1Hub**](scaleway.iot.v1.Hub.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hub_ca

> crate::models::ScalewayIotV1GetHubCaResponse get_hub_ca(region, hub_id)
Get the certificate authority of a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**hub_id** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayIotV1GetHubCaResponse**](scaleway.iot.v1.GetHubCAResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hub_metrics

> crate::models::ScalewayIotV1GetHubMetricsResponse get_hub_metrics(region, hub_id, start_date)
Get a hub's metrics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**hub_id** | **String** | Hub ID | [required] |
**start_date** | **String** | Start date used to compute the best scale for the returned metrics | [required] |

### Return type

[**crate::models::ScalewayIotV1GetHubMetricsResponse**](scaleway.iot.v1.GetHubMetricsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_hubs

> crate::models::ScalewayIotV1ListHubsResponse list_hubs(region, page, page_size, order_by, project_id, organization_id, name)
List hubs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | Option<**i64**> | Page number |  |[default to 1]
**page_size** | Option<**i64**> | Page size. The maximum value is 100 |  |[default to 20]
**order_by** | Option<**String**> | Ordering of requested hub |  |[default to name_asc]
**project_id** | Option<**String**> | Filter on project |  |
**organization_id** | Option<**String**> | Filter on the organization |  |
**name** | Option<**String**> | Filter on the name |  |

### Return type

[**crate::models::ScalewayIotV1ListHubsResponse**](scaleway.iot.v1.ListHubsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_hub_ca

> crate::models::ScalewayIotV1Hub set_hub_ca(region, hub_id, set_hub_ca_request)
Set the certificate authority of a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**hub_id** | **String** | Hub ID | [required] |
**set_hub_ca_request** | [**SetHubCaRequest**](SetHubCaRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Hub**](scaleway.iot.v1.Hub.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_hub

> crate::models::ScalewayIotV1Hub update_hub(region, hub_id, update_hub_request)
Update a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**hub_id** | **String** | Hub ID | [required] |
**update_hub_request** | [**UpdateHubRequest**](UpdateHubRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Hub**](scaleway.iot.v1.Hub.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

