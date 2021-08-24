# \DefaultApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_service_info**](DefaultApi.md#get_service_info) | **GET** /rdb/v1/regions/{region} | 
[**move_mac_addr**](DefaultApi.md#move_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/move | 
[**renew_instance_certificate**](DefaultApi.md#renew_instance_certificate) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/renew-certificate | 



## get_service_info

> crate::models::ScalewayStdServiceInfo get_service_info(region)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |

### Return type

[**crate::models::ScalewayStdServiceInfo**](scaleway.std.ServiceInfo.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_mac_addr

> crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp move_mac_addr(zone, fip_id, inline_object43)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** |  | [required] |
**inline_object43** | [**InlineObject43**](InlineObject43.md) |  | [required] |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## renew_instance_certificate

> renew_instance_certificate(region, instance_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

