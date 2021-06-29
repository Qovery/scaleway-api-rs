# \IoTHubsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_hub**](IoTHubsApi.md#create_hub) | **post** /iot/v1/regions/{region}/hubs | Create a hub
[**delete_hub**](IoTHubsApi.md#delete_hub) | **delete** /iot/v1/regions/{region}/hubs/{hub_id} | Delete a hub
[**disable_hub**](IoTHubsApi.md#disable_hub) | **post** /iot/v1/regions/{region}/hubs/{hub_id}/disable | Disable a hub
[**enable_hub**](IoTHubsApi.md#enable_hub) | **post** /iot/v1/regions/{region}/hubs/{hub_id}/enable | Enable a hub
[**get_hub**](IoTHubsApi.md#get_hub) | **get** /iot/v1/regions/{region}/hubs/{hub_id} | Get a hub
[**get_hub_ca**](IoTHubsApi.md#get_hub_ca) | **get** /iot/v1/regions/{region}/hubs/{hub_id}/ca | Get the certificate authority of a hub
[**get_hub_metrics**](IoTHubsApi.md#get_hub_metrics) | **get** /iot/v1/regions/{region}/hubs/{hub_id}/metrics | Get a hub's metrics
[**list_hubs**](IoTHubsApi.md#list_hubs) | **get** /iot/v1/regions/{region}/hubs | List hubs
[**set_hub_ca**](IoTHubsApi.md#set_hub_ca) | **post** /iot/v1/regions/{region}/hubs/{hub_id}/ca | Set the certificate authority of a hub
[**update_hub**](IoTHubsApi.md#update_hub) | **patch** /iot/v1/regions/{region}/hubs/{hub_id} | Update a hub



## create_hub

> crate::models::ScalewayIotV1Hub create_hub(region, inline_object85)
Create a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**inline_object85** | [**InlineObject85**](InlineObject85.md) |  | [required] |

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
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | Page size. The maximum value is 100 |  |[default to 20]
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

> crate::models::ScalewayIotV1Hub set_hub_ca(region, hub_id, inline_object87)
Set the certificate authority of a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**hub_id** | **String** | Hub ID | [required] |
**inline_object87** | [**InlineObject87**](InlineObject87.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Hub**](scaleway.iot.v1.Hub.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_hub

> crate::models::ScalewayIotV1Hub update_hub(region, hub_id, inline_object86)
Update a hub

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**hub_id** | **String** | Hub ID | [required] |
**inline_object86** | [**InlineObject86**](InlineObject86.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Hub**](scaleway.iot.v1.Hub.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

