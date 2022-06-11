# \DefaultApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**move_mac_addr**](DefaultApi.md#move_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/move | 



## move_mac_addr

> crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp move_mac_addr(zone, fip_id, move_mac_addr_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** |  | [required] |
**move_mac_addr_request** | [**MoveMacAddrRequest**](MoveMacAddrRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

