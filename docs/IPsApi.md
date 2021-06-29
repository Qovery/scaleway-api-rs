# \IpsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ip1**](IpsApi.md#create_ip1) | **post** /lb/v1/regions/{region}/ips | Create an IP
[**get_ip1**](IpsApi.md#get_ip1) | **get** /lb/v1/regions/{region}/ips/{ip_id} | Get an IP
[**list_ips**](IpsApi.md#list_ips) | **get** /lb/v1/regions/{region}/ips | List IPs
[**release_ip**](IpsApi.md#release_ip) | **delete** /lb/v1/regions/{region}/ips/{ip_id} | Delete an IP
[**update_ip1**](IpsApi.md#update_ip1) | **patch** /lb/v1/regions/{region}/ips/{ip_id} | Update an IP



## create_ip1

> crate::models::ScalewayLbV1Ip create_ip1(region, inline_object38)
Create an IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**inline_object38** | [**InlineObject38**](InlineObject38.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Ip**](scaleway.lb.v1.Ip.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ip1

> crate::models::ScalewayLbV1Ip get_ip1(region, ip_id)
Get an IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**ip_id** | **String** | IP address ID | [required] |

### Return type

[**crate::models::ScalewayLbV1Ip**](scaleway.lb.v1.Ip.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ips

> crate::models::ScalewayLbV1ListIpsResponse list_ips(region, page, page_size, ip_address, organization_id, project_id)
List IPs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | **f32** | Page number | [required] |[default to 1]
**page_size** | Option<**f32**> | The number of items to return |  |[default to 20]
**ip_address** | Option<**String**> | Use this to search by IP address |  |
**organization_id** | Option<**String**> | Filter IPs by organization id |  |
**project_id** | Option<**String**> | Filter IPs by project ID |  |

### Return type

[**crate::models::ScalewayLbV1ListIpsResponse**](scaleway.lb.v1.ListIpsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## release_ip

> release_ip(region, ip_id)
Delete an IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**ip_id** | **String** | IP address ID | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ip1

> crate::models::ScalewayLbV1Ip update_ip1(region, ip_id, inline_object39)
Update an IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**ip_id** | **String** | IP address ID | [required] |
**inline_object39** | [**InlineObject39**](InlineObject39.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Ip**](scaleway.lb.v1.Ip.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

