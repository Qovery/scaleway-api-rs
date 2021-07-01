# \DefaultApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dashboard**](DefaultApi.md#get_dashboard) | **get** /instance/v1/zones/{zone}/dashboard | 
[**get_service_info**](DefaultApi.md#get_service_info) | **get** /apple-silicon/v1alpha1/zones/{zone} | 
[**move_mac_addr**](DefaultApi.md#move_mac_addr) | **post** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/move | 



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

> crate::models::ScalewayStdServiceInfo get_service_info(zone)


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


## move_mac_addr

> crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp move_mac_addr(zone, fip_id, inline_object39)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** |  | [required] |
**inline_object39** | [**InlineObject39**](InlineObject39.md) |  | [required] |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

