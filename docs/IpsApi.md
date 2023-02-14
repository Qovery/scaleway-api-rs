# \IpsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ip**](IpsApi.md#create_ip) | **POST** /instance/v1/zones/{zone}/ips | Reserve a flexible IP
[**delete_ip**](IpsApi.md#delete_ip) | **DELETE** /instance/v1/zones/{zone}/ips/{ip} | Delete a flexible IP
[**get_ip**](IpsApi.md#get_ip) | **GET** /instance/v1/zones/{zone}/ips/{ip} | Get a flexible IP
[**list_ips**](IpsApi.md#list_ips) | **GET** /instance/v1/zones/{zone}/ips | List all flexible IPs
[**update_ip**](IpsApi.md#update_ip) | **PATCH** /instance/v1/zones/{zone}/ips/{ip} | Update a flexible IP



## create_ip

> crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateIpResponse create_ip(zone, create_ip_request)
Reserve a flexible IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_ip_request** | [**CreateIpRequest**](CreateIpRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateIpResponse**](scaleway.instance.v1.CreateIpResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ip

> delete_ip(zone, ip)
Delete a flexible IP

Delete the IP with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**ip** | **String** | The ID or the address of the IP to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ip

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetIpResponse get_ip(zone, ip)
Get a flexible IP

Get details of an IP with the given ID or address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**ip** | **String** | The IP ID or address to get | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetIpResponse**](scaleway.instance.v1.GetIpResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ips

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListIpsResponse list_ips(zone, project, organization, tags, name, per_page, page)
List all flexible IPs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**project** | Option<**String**> | The project ID the IPs are reserved in |  |
**organization** | Option<**String**> | The organization ID the IPs are reserved in |  |
**tags** | Option<**String**> | Filter IPs with these exact tags (to filter with several tags, use commas to separate them) |  |
**name** | Option<**String**> | Filter on the IP address (Works as a LIKE operation on the IP address) |  |
**per_page** | Option<**i32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**i32**> | A positive integer to choose the page to return |  |[default to 1]

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListIpsResponse**](scaleway.instance.v1.ListIpsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ip

> crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdateIpResponse update_ip(zone, ip, update_ip_request)
Update a flexible IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**ip** | **String** | IP ID or IP address | [required] |
**update_ip_request** | [**UpdateIpRequest**](UpdateIpRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdateIpResponse**](scaleway.instance.v1.UpdateIpResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

