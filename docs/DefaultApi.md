# \DefaultApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_policy**](DefaultApi.md#clone_policy) | **POST** /iam/v1alpha1/policies/{policy_id}/clone | 
[**get_dashboard**](DefaultApi.md#get_dashboard) | **GET** /instance/v1/zones/{zone}/dashboard | 
[**move_mac_addr**](DefaultApi.md#move_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/move | 



## clone_policy

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodPolicy clone_policy(policy_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodPolicy**](scaleway.iam.v1alpha1.Policy.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetDashboardResponse get_dashboard(zone, organization, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**organization** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetDashboardResponse**](scaleway.instance.v1.GetDashboardResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_mac_addr

> crate::models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp move_mac_addr(zone, fip_id, move_mac_addr_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** |  | [required] |
**move_mac_addr_request** | [**MoveMacAddrRequest**](MoveMacAddrRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

