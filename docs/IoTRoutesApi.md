# \IoTRoutesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_route**](IoTRoutesApi.md#create_route) | **POST** /iot/v1/regions/{region}/routes | Create a route
[**delete_route**](IoTRoutesApi.md#delete_route) | **DELETE** /iot/v1/regions/{region}/routes/{route_id} | Delete a route
[**get_route**](IoTRoutesApi.md#get_route) | **GET** /iot/v1/regions/{region}/routes/{route_id} | Get a route
[**list_routes**](IoTRoutesApi.md#list_routes) | **GET** /iot/v1/regions/{region}/routes | List routes
[**update_route**](IoTRoutesApi.md#update_route) | **PATCH** /iot/v1/regions/{region}/routes/{route_id} | Update a route



## create_route

> crate::models::ScalewayIotV1Route create_route(region, inline_object46)
Create a route

Multiple route kinds can be created: - Database Route.   Create a route that will record subscribed MQTT messages into your database.   <b>You need to manage the database by yourself</b>. - REST Route.   Create a route that will call a REST API on received subscribed MQTT messages. - S3 Routes.   Create a route that will put subscribed MQTT messages into an S3 bucket.   You need to create the bucket yourself and grant us write access.   The grant can be done with s3cmd (`s3cmd setacl s3://<my-bucket> --acl-grant=write:555c69c3-87d0-4bf8-80f1-99a2f757d031:555c69c3-87d0-4bf8-80f1-99a2f757d031`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**inline_object46** | [**InlineObject46**](InlineObject46.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Route**](scaleway.iot.v1.Route.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_route

> delete_route(region, route_id)
Delete a route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**route_id** | **String** | Route ID | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_route

> crate::models::ScalewayIotV1Route get_route(region, route_id)
Get a route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**route_id** | **String** | Route ID | [required] |

### Return type

[**crate::models::ScalewayIotV1Route**](scaleway.iot.v1.Route.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_routes

> crate::models::ScalewayIotV1ListRoutesResponse list_routes(region, page, page_size, order_by, hub_id, name)
List routes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | Page size. The maximum value is 100 |  |[default to 20]
**order_by** | Option<**String**> | Ordering of requested routes |  |[default to name_asc]
**hub_id** | Option<**String**> | Filter on the hub |  |
**name** | Option<**String**> | Filter on route's name |  |

### Return type

[**crate::models::ScalewayIotV1ListRoutesResponse**](scaleway.iot.v1.ListRoutesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_route

> crate::models::ScalewayIotV1Route update_route(region, route_id, inline_object47)
Update a route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**route_id** | **String** | Route id | [required] |
**inline_object47** | [**InlineObject47**](InlineObject47.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Route**](scaleway.iot.v1.Route.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

