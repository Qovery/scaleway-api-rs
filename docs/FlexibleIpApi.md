# \FlexibleIpApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_flexible_ip**](FlexibleIpApi.md#attach_flexible_ip) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/attach | Attach an existing flexible IP to a server
[**create_flexible_ip**](FlexibleIpApi.md#create_flexible_ip) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips | Create a new flexible IP
[**delete_flexible_ip**](FlexibleIpApi.md#delete_flexible_ip) | **DELETE** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id} | Delete an existing flexible IP
[**delete_mac_addr**](FlexibleIpApi.md#delete_mac_addr) | **DELETE** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac | Detach a given virtual MAC address from an existing flexible IP
[**detach_flexible_ip**](FlexibleIpApi.md#detach_flexible_ip) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/detach | Detach an existing flexible IP from a server
[**duplicate_mac_addr**](FlexibleIpApi.md#duplicate_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/duplicate | Duplicate a virtual MAC address to another flexible IP
[**generate_mac_addr**](FlexibleIpApi.md#generate_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac | Generate a virtual MAC address on an existing flexible IP
[**get_flexible_ip**](FlexibleIpApi.md#get_flexible_ip) | **GET** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id} | Get an existing flexible IP
[**list_flexible_ips**](FlexibleIpApi.md#list_flexible_ips) | **GET** /flexible-ip/v1alpha1/zones/{zone}/fips | List flexible IPs
[**move_mac_addr**](FlexibleIpApi.md#move_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/move | Relocate an existing virtual MAC address to a different flexible IP
[**update_flexible_ip**](FlexibleIpApi.md#update_flexible_ip) | **PATCH** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id} | Update an existing flexible IP



## attach_flexible_ip

> models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse attach_flexible_ip(zone, attach_flexible_ip_request)
Attach an existing flexible IP to a server

Attach an existing flexible IP to a specified Elastic Metal server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**attach_flexible_ip_request** | [**AttachFlexibleIpRequest**](AttachFlexibleIpRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse**](scaleway.flexible_ip.v1alpha1.AttachFlexibleIPsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_flexible_ip

> models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp create_flexible_ip(zone, create_flexible_ip_request)
Create a new flexible IP

Generate a new flexible IP within a given zone, specifying its configuration including Project ID and description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_flexible_ip_request** | [**CreateFlexibleIpRequest**](CreateFlexibleIpRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_flexible_ip

> delete_flexible_ip(zone, fip_id)
Delete an existing flexible IP

Delete an existing flexible IP, specified by its ID and zone. Note that deleting a flexible IP is permanent and cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | ID of the flexible IP to delete. | [required] |

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
Detach a given virtual MAC address from an existing flexible IP

Detach a given MAC (Media Access Control) address from an existing flexible IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | ID of the flexible IP from which to delete the virtual MAC. If the flexible IP belongs to a MAC group, the MAC will be removed from both the MAC group and flexible IP. | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_flexible_ip

> models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodDetachFlexibleIpsResponse detach_flexible_ip(zone, detach_flexible_ip_request)
Detach an existing flexible IP from a server

Detach an existing flexible IP from a specified Elastic Metal server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**detach_flexible_ip_request** | [**DetachFlexibleIpRequest**](DetachFlexibleIpRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodDetachFlexibleIpsResponse**](scaleway.flexible_ip.v1alpha1.DetachFlexibleIPsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## duplicate_mac_addr

> models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp duplicate_mac_addr(zone, fip_id, duplicate_mac_addr_request)
Duplicate a virtual MAC address to another flexible IP

Duplicate a virtual MAC address from a given flexible IP to another flexible IP attached to the same server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | ID of the flexible IP on which to duplicate the virtual MAC. Note that the flexible IPs need to be attached to the same server. | [required] |
**duplicate_mac_addr_request** | [**DuplicateMacAddrRequest**](DuplicateMacAddrRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_mac_addr

> models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp generate_mac_addr(zone, fip_id, generate_mac_addr_request)
Generate a virtual MAC address on an existing flexible IP

Generate a virtual MAC (Media Access Control) address on an existing flexible IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | ID of the flexible IP for which to generate a virtual MAC. | [required] |
**generate_mac_addr_request** | [**GenerateMacAddrRequest**](GenerateMacAddrRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flexible_ip

> models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp get_flexible_ip(zone, fip_id)
Get an existing flexible IP

Retrieve information about an existing flexible IP, specified by its ID and zone. Its full details, including Project ID, description and status, are returned in the response object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | ID of the flexible IP. | [required] |

### Return type

[**models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_flexible_ips

> models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodListFlexibleIpsResponse list_flexible_ips(zone, order_by, page, page_size, tags, status, server_ids, organization_id, project_id)
List flexible IPs

List all flexible IPs within a given zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**order_by** | Option<**String**> | Sort order of the returned flexible IPs. |  |[default to created_at_asc]
**page** | Option<**i32**> | Page number. |  |
**page_size** | Option<**i32**> | Maximum number of flexible IPs per page. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Filter by tag, only flexible IPs with one or more matching tags will be returned. |  |
**status** | Option<[**Vec<models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIpPeriodStatus>**](models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIpPeriodStatus.md)> | Filter by status, only flexible IPs with this status will be returned. |  |
**server_ids** | Option<[**Vec<String>**](String.md)> | Filter by server IDs, only flexible IPs with these server IDs will be returned. |  |
**organization_id** | Option<**String**> | Filter by Organization ID, only flexible IPs from this Organization will be returned. |  |
**project_id** | Option<**String**> | Filter by Project ID, only flexible IPs from this Project will be returned. |  |

### Return type

[**models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodListFlexibleIpsResponse**](scaleway.flexible_ip.v1alpha1.ListFlexibleIPsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_mac_addr

> models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp move_mac_addr(zone, fip_id, move_mac_addr_request)
Relocate an existing virtual MAC address to a different flexible IP

Relocate a virtual MAC (Media Access Control) address from an existing flexible IP to a different flexible IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** |  | [required] |
**move_mac_addr_request** | [**MoveMacAddrRequest**](MoveMacAddrRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_flexible_ip

> models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp update_flexible_ip(zone, fip_id, update_flexible_ip_request)
Update an existing flexible IP

Update the parameters of an existing flexible IP, specified by its ID and zone. These parameters include tags and description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**fip_id** | **String** | ID of the flexible IP to update. | [required] |
**update_flexible_ip_request** | [**UpdateFlexibleIpRequest**](UpdateFlexibleIpRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp**](scaleway.flexible_ip.v1alpha1.FlexibleIP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

