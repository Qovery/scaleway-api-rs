# \IPsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ip**](IPsApi.md#create_ip) | **post** /instance/v1/zones/{zone}/ips | Reserve a flexible IP
[**delete_ip**](IPsApi.md#delete_ip) | **delete** /instance/v1/zones/{zone}/ips/{ip} | Delete a flexible IP
[**get_ip**](IPsApi.md#get_ip) | **get** /instance/v1/zones/{zone}/ips/{ip} | Get a flexible IP
[**list_ips**](IPsApi.md#list_ips) | **get** /instance/v1/zones/{zone}/ips | List all flexible IPs
[**update_ip**](IPsApi.md#update_ip) | **patch** /instance/v1/zones/{zone}/ips/{ip} | Update a flexible IP



## create_ip

> crate::models::ScalewayInstanceV1CreateIpResponse create_ip(zone, inline_object2)
Reserve a flexible IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object2** | [**InlineObject2**](InlineObject2.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1CreateIpResponse**](scaleway.instance.v1.CreateIpResponse.md)

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

> crate::models::ScalewayInstanceV1GetIpResponse get_ip(zone, ip)
Get a flexible IP

Get details of an IP with the given ID or address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**ip** | **String** | The IP ID or address to get | [required] |

### Return type

[**crate::models::ScalewayInstanceV1GetIpResponse**](scaleway.instance.v1.GetIpResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ips

> crate::models::ScalewayInstanceV1ListIpsResponse list_ips(zone, organization, name, per_page, page, project)
List all flexible IPs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**organization** | Option<**String**> | The organization ID the IPs are reserved in |  |
**name** | Option<**String**> | Filter on the IP address (Works as a LIKE operation on the IP address) |  |
**per_page** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**f32**> | A positive integer to choose the page to return |  |[default to 1]
**project** | Option<**String**> | The project ID the IPs are reserved in |  |

### Return type

[**crate::models::ScalewayInstanceV1ListIpsResponse**](scaleway.instance.v1.ListIpsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ip

> crate::models::ScalewayInstanceV1UpdateIpResponse update_ip(zone, ip, inline_object3)
Update a flexible IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**ip** | **String** | IP ID or IP address | [required] |
**inline_object3** | [**InlineObject3**](InlineObject3.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1UpdateIpResponse**](scaleway.instance.v1.UpdateIpResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

