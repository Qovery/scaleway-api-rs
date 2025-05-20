# \VersionsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dns_zone_version_diff**](VersionsApi.md#get_dns_zone_version_diff) | **GET** /domain/v2beta1/dns-zones/version/{dns_zone_version_id}/diff | Access differences from a specific DNS zone version
[**get_version**](VersionsApi.md#get_version) | **GET** /k8s/v1/regions/{region}/versions/{version_name} | Get a Version
[**list_dns_zone_version_records**](VersionsApi.md#list_dns_zone_version_records) | **GET** /domain/v2beta1/dns-zones/version/{dns_zone_version_id} | List records from a given version of a specific DNS zone
[**list_dns_zone_versions**](VersionsApi.md#list_dns_zone_versions) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/versions | List versions of a DNS zone
[**list_versions**](VersionsApi.md#list_versions) | **GET** /k8s/v1/regions/{region}/versions | List all available Versions
[**restore_dns_zone_version**](VersionsApi.md#restore_dns_zone_version) | **POST** /domain/v2beta1/dns-zones/version/{dns_zone_version_id}/restore | Restore a DNS zone version



## get_dns_zone_version_diff

> models::ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneVersionDiffResponse get_dns_zone_version_diff(dns_zone_version_id)
Access differences from a specific DNS zone version

Access a previous DNS zone version to see the differences from another specific version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_version_id** | **String** | (UUID format) | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneVersionDiffResponse**](scaleway.domain.v2beta1.GetDNSZoneVersionDiffResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version

> models::ScalewayPeriodK8sPeriodV1PeriodVersion get_version(region, version_name)
Get a Version

Retrieve a specific Kubernetes version and its details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**version_name** | **String** | Requested version name. | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodVersion**](scaleway.k8s.v1.Version.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_zone_version_records

> models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionRecordsResponse list_dns_zone_version_records(dns_zone_version_id, page, page_size)
List records from a given version of a specific DNS zone

Retrieve a list of records from a specific DNS zone version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_version_id** | **String** | (UUID format) | [required] |
**page** | Option<**i32**> | Page number to return, from the paginated results. |  |
**page_size** | Option<**i32**> | Maximum number of DNS zones versions records per page. |  |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionRecordsResponse**](scaleway.domain.v2beta1.ListDNSZoneVersionRecordsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_zone_versions

> models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionsResponse list_dns_zone_versions(dns_zone, page, page_size)
List versions of a DNS zone

Retrieve a list of a DNS zone's versions.<br/> The maximum version count is 100. If the count reaches this limit, the oldest version will be deleted after each new modification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |
**page** | Option<**i32**> | Page number to return, from the paginated results. |  |
**page_size** | Option<**i32**> | Maximum number of DNS zones versions per page. |  |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionsResponse**](scaleway.domain.v2beta1.ListDNSZoneVersionsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_versions

> models::ScalewayPeriodK8sPeriodV1PeriodListVersionsResponse list_versions(region)
List all available Versions

List all available versions for the creation of a new Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodListVersionsResponse**](scaleway.k8s.v1.ListVersionsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_dns_zone_version

> serde_json::Value restore_dns_zone_version(dns_zone_version_id, body)
Restore a DNS zone version

Restore and activate a version of a specific DNS zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_version_id** | **String** | (UUID format) | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

