# \IoTDevicesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_device**](IoTDevicesApi.md#create_device) | **POST** /iot/v1/regions/{region}/devices | Add a device
[**delete_device**](IoTDevicesApi.md#delete_device) | **DELETE** /iot/v1/regions/{region}/devices/{device_id} | Remove a device
[**disable_device**](IoTDevicesApi.md#disable_device) | **POST** /iot/v1/regions/{region}/devices/{device_id}/disable | Disable a device
[**enable_device**](IoTDevicesApi.md#enable_device) | **POST** /iot/v1/regions/{region}/devices/{device_id}/enable | Enable a device
[**get_device**](IoTDevicesApi.md#get_device) | **GET** /iot/v1/regions/{region}/devices/{device_id} | Get a device
[**get_device_certificate**](IoTDevicesApi.md#get_device_certificate) | **GET** /iot/v1/regions/{region}/devices/{device_id}/certificate | Get a device's certificate
[**get_device_metrics**](IoTDevicesApi.md#get_device_metrics) | **GET** /iot/v1/regions/{region}/devices/{device_id}/metrics | Get a device's metrics
[**list_devices**](IoTDevicesApi.md#list_devices) | **GET** /iot/v1/regions/{region}/devices | List devices
[**renew_device_certificate**](IoTDevicesApi.md#renew_device_certificate) | **POST** /iot/v1/regions/{region}/devices/{device_id}/renew-certificate | Renew a device certificate
[**set_device_certificate**](IoTDevicesApi.md#set_device_certificate) | **PUT** /iot/v1/regions/{region}/devices/{device_id}/certificate | Set a custom certificate on a device
[**update_device**](IoTDevicesApi.md#update_device) | **PATCH** /iot/v1/regions/{region}/devices/{device_id} | Update a device



## create_device

> crate::models::ScalewayIotV1CreateDeviceResponse create_device(region, create_device_request)
Add a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**create_device_request** | [**CreateDeviceRequest**](CreateDeviceRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1CreateDeviceResponse**](scaleway.iot.v1.CreateDeviceResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_device

> delete_device(region, device_id)
Remove a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**device_id** | **String** | Device ID | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_device

> crate::models::ScalewayIotV1Device disable_device(region, device_id, body)
Disable a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**device_id** | **String** | Device ID | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Device**](scaleway.iot.v1.Device.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_device

> crate::models::ScalewayIotV1Device enable_device(region, device_id, body)
Enable a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**device_id** | **String** | Device ID | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Device**](scaleway.iot.v1.Device.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device

> crate::models::ScalewayIotV1Device get_device(region, device_id)
Get a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**device_id** | **String** | Device ID | [required] |

### Return type

[**crate::models::ScalewayIotV1Device**](scaleway.iot.v1.Device.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device_certificate

> crate::models::ScalewayIotV1GetDeviceCertificateResponse get_device_certificate(region, device_id)
Get a device's certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**device_id** | **String** | Device ID | [required] |

### Return type

[**crate::models::ScalewayIotV1GetDeviceCertificateResponse**](scaleway.iot.v1.GetDeviceCertificateResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device_metrics

> crate::models::ScalewayIotV1GetDeviceMetricsResponse get_device_metrics(region, device_id, start_date)
Get a device's metrics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**device_id** | **String** | Device ID | [required] |
**start_date** | **String** | Start date used to compute the best scale for the returned metrics | [required] |

### Return type

[**crate::models::ScalewayIotV1GetDeviceMetricsResponse**](scaleway.iot.v1.GetDeviceMetricsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_devices

> crate::models::ScalewayIotV1ListDevicesResponse list_devices(region, page, page_size, order_by, name, hub_id, allow_insecure, status)
List devices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | Option<**i64**> | Page number |  |[default to 1]
**page_size** | Option<**i64**> | Page size. The maximum value is 100 |  |[default to 20]
**order_by** | Option<**String**> | Ordering of requested devices |  |[default to name_asc]
**name** | Option<**String**> | Filter on the name |  |
**hub_id** | Option<**String**> | Filter on the hub |  |
**allow_insecure** | Option<**bool**> | Filter on the allow_insecure flag |  |
**status** | Option<**String**> | Device status (enabled, disabled, etc.) |  |[default to unknown]

### Return type

[**crate::models::ScalewayIotV1ListDevicesResponse**](scaleway.iot.v1.ListDevicesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## renew_device_certificate

> crate::models::ScalewayIotV1RenewDeviceCertificateResponse renew_device_certificate(region, device_id, body)
Renew a device certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**device_id** | **String** | Device ID | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayIotV1RenewDeviceCertificateResponse**](scaleway.iot.v1.RenewDeviceCertificateResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_device_certificate

> crate::models::ScalewayIotV1SetDeviceCertificateResponse set_device_certificate(region, device_id, set_device_certificate_request)
Set a custom certificate on a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**device_id** | **String** | Device ID | [required] |
**set_device_certificate_request** | [**SetDeviceCertificateRequest**](SetDeviceCertificateRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1SetDeviceCertificateResponse**](scaleway.iot.v1.SetDeviceCertificateResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_device

> crate::models::ScalewayIotV1Device update_device(region, device_id, update_device_request)
Update a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**device_id** | **String** | Device ID | [required] |
**update_device_request** | [**UpdateDeviceRequest**](UpdateDeviceRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1Device**](scaleway.iot.v1.Device.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

