# \DefaultApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dashboard**](DefaultApi.md#get_dashboard) | **get** /instance/v1/zones/{zone}/dashboard | 
[**get_service_info**](DefaultApi.md#get_service_info) | **get** /rdb/v1/regions/{region} | 
[**get_service_info1**](DefaultApi.md#get_service_info1) | **get** /apple-silicon/v1alpha1/zones/{zone} | 
[**list_backend_stats**](DefaultApi.md#list_backend_stats) | **get** /lb/v1/regions/{region}/lbs/{lb_id}/backend-stats | 
[**move_mac_addr**](DefaultApi.md#move_mac_addr) | **post** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/move | 
[**renew_instance_certificate**](DefaultApi.md#renew_instance_certificate) | **post** /rdb/v1/regions/{region}/instances/{instance_id}/renew-certificate | 



## get_dashboard

> crate::models::ScalewayInstanceV1GetDashboardResponse get_dashboard(zone, organization, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**organization** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |

### Return type

[**crate::models::ScalewayInstanceV1GetDashboardResponse**](scaleway.instance.v1.GetDashboardResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## get_service_info1

> crate::models::ScalewayStdServiceInfo get_service_info1(zone)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |

### Return type

[**crate::models::ScalewayStdServiceInfo**](scaleway.std.ServiceInfo.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_backend_stats

> crate::models::ScalewayLbV1ListBackendStatsResponse list_backend_stats(region, lb_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | The number of items to return |  |[default to 20]

### Return type

[**crate::models::ScalewayLbV1ListBackendStatsResponse**](scaleway.lb.v1.ListBackendStatsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_mac_addr

> crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp move_mac_addr(zone, fip_id, inline_object99)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** |  | [required] |
**inline_object99** | [**InlineObject99**](InlineObject99.md) |  | [required] |

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

