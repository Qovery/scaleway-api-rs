# \FlexibleIPApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_flexible_ip**](FlexibleIPApi.md#attach_flexible_ip) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/attach | Attach a Flexible IP to a server
[**create_flexible_ip**](FlexibleIPApi.md#create_flexible_ip) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips | Create a Flexible IP
[**delete_flexible_ip**](FlexibleIPApi.md#delete_flexible_ip) | **DELETE** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id} | Delete a Flexible IP
[**delete_mac_addr**](FlexibleIPApi.md#delete_mac_addr) | **DELETE** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac | Remove a virtual MAC from a Flexible IP
[**detach_flexible_ip**](FlexibleIPApi.md#detach_flexible_ip) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/detach | Detach a Flexible IP from a server
[**duplicate_mac_addr**](FlexibleIPApi.md#duplicate_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/duplicate | Duplicate a Virtual MAC
[**generate_mac_addr**](FlexibleIPApi.md#generate_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac | Generate a virtual MAC on a given Flexible IP
[**get_flexible_ip**](FlexibleIPApi.md#get_flexible_ip) | **GET** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id} | Get a Flexible IP
[**list_flexible_ips**](FlexibleIPApi.md#list_flexible_ips) | **GET** /flexible-ip/v1alpha1/zones/{zone}/fips | List Flexible IPs
[**update_flexible_ip**](FlexibleIPApi.md#update_flexible_ip) | **PATCH** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id} | Update a Flexible IP



## attach_flexible_ip

> crate::models::ScalewayFlexibleIpV1alpha1AttachFlexibleIpsResponse attach_flexible_ip(zone, inline_object44)
Attach a Flexible IP to a server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object44** | [**InlineObject44**](InlineObject44.md) |  | [required] |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1AttachFlexibleIpsResponse**](scaleway.flexible_ip.v1alpha1.AttachFlexibleIPsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_flexible_ip

> crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp create_flexible_ip(zone, inline_object39)
Create a Flexible IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object39** | [**InlineObject39**](InlineObject39.md) |  | [required] |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_flexible_ip

> delete_flexible_ip(zone, fip_id)
Delete a Flexible IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | ID of the Flexible IP to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mac_addr

> delete_mac_addr(zone, fip_id)
Remove a virtual MAC from a Flexible IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | If the Flexible IP belongs to a MAC group, the MAC will be removed from the MAC group and from the Flexible IP. | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_flexible_ip

> crate::models::ScalewayFlexibleIpV1alpha1DetachFlexibleIpsResponse detach_flexible_ip(zone, inline_object45)
Detach a Flexible IP from a server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object45** | [**InlineObject45**](InlineObject45.md) |  | [required] |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1DetachFlexibleIpsResponse**](scaleway.flexible_ip.v1alpha1.DetachFlexibleIPsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## duplicate_mac_addr

> crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp duplicate_mac_addr(zone, fip_id, inline_object42)
Duplicate a Virtual MAC

Duplicate a Virtual MAC from a given Flexible IP onto another attached on the same server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | Flexible IPs need to be attached to the same server. | [required] |
**inline_object42** | [**InlineObject42**](InlineObject42.md) |  | [required] |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_mac_addr

> crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp generate_mac_addr(zone, fip_id, inline_object41)
Generate a virtual MAC on a given Flexible IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | Flexible IP ID on which to generate a Virtual MAC | [required] |
**inline_object41** | [**InlineObject41**](InlineObject41.md) |  | [required] |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flexible_ip

> crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp get_flexible_ip(zone, fip_id)
Get a Flexible IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | Flexible IP ID | [required] |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_flexible_ips

> crate::models::ScalewayFlexibleIpV1alpha1ListFlexibleIpsResponse list_flexible_ips(zone, order_by, page, page_size, tags, status, server_ids, organization_id, project_id)
List Flexible IPs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**order_by** | Option<**String**> | The sort order of the returned Flexible IPs |  |[default to created_at_asc]
**page** | Option<**f32**> | The page number for the returned Flexible IPs |  |[default to 1]
**page_size** | Option<**f32**> | The maximum number of Flexible IPs per page |  |[default to 20]
**tags** | Option<[**Vec<String>**](String.md)> | Filter Flexible IPs with one or more matching tags |  |
**status** | Option<[**Vec<crate::models::ScalewayFlexibleIpV1alpha1FlexibleIpStatus>**](crate::models::ScalewayFlexibleIpV1alpha1FlexibleIpStatus.md)> | Filter Flexible IPs by status |  |
**server_ids** | Option<[**Vec<String>**](String.md)> | Filter Flexible IPs by server IDs |  |
**organization_id** | Option<**String**> | Filter Flexible IPs by organization ID |  |
**project_id** | Option<**String**> | Filter Flexible IPs by project ID |  |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1ListFlexibleIpsResponse**](scaleway.flexible_ip.v1alpha1.ListFlexibleIPsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_flexible_ip

> crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp update_flexible_ip(zone, fip_id, inline_object40)
Update a Flexible IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | ID of the Flexible IP to update | [required] |
**inline_object40** | [**InlineObject40**](InlineObject40.md) |  | [required] |

### Return type

[**crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

