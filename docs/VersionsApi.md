# \VersionsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dns_zone_version_diff**](VersionsApi.md#get_dns_zone_version_diff) | **GET** /domain/v2beta1/dns-zones/version/{dns_zone_version_id}/diff | Get DNS zone version diff
[**get_version**](VersionsApi.md#get_version) | **GET** /k8s/v1/regions/{region}/versions/{version_name} | Get details about a specific version
[**list_dns_zone_version_records**](VersionsApi.md#list_dns_zone_version_records) | **GET** /domain/v2beta1/dns-zones/version/{dns_zone_version_id} | List DNS zone version records
[**list_dns_zone_versions**](VersionsApi.md#list_dns_zone_versions) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/versions | List DNS zone versions
[**list_versions**](VersionsApi.md#list_versions) | **GET** /k8s/v1/regions/{region}/versions | List all available versions
[**restore_dns_zone_version**](VersionsApi.md#restore_dns_zone_version) | **POST** /domain/v2beta1/dns-zones/version/{dns_zone_version_id}/restore | Restore DNS zone version



## get_dns_zone_version_diff

> crate::models::ScalewayDomainV2beta1GetDnsZoneVersionDiffResponse get_dns_zone_version_diff(dns_zone_version_id)
Get DNS zone version diff

Get all differences from a previous DNS zone version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_version_id** | **String** | (UUID format) | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1GetDnsZoneVersionDiffResponse**](scaleway.domain.v2beta1.GetDNSZoneVersionDiffResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version

> crate::models::ScalewayK8sV1Version get_version(region, version_name)
Get details about a specific version

This method allows to get a specific Kubernetes version and the details about the version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**version_name** | **String** | The requested version name | [required] |

### Return type

[**crate::models::ScalewayK8sV1Version**](scaleway.k8s.v1.Version.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_zone_version_records

> crate::models::ScalewayDomainV2beta1ListDnsZoneVersionRecordsResponse list_dns_zone_version_records(dns_zone_version_id)
List DNS zone version records

Get a list of records from a previous DNS zone version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_version_id** | **String** | (UUID format) | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1ListDnsZoneVersionRecordsResponse**](scaleway.domain.v2beta1.ListDNSZoneVersionRecordsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_zone_versions

> crate::models::ScalewayDomainV2beta1ListDnsZoneVersionsResponse list_dns_zone_versions(dns_zone)
List DNS zone versions

Get a list of DNS zone versions.<br/> The maximum version count is 100.<br/> If the count reaches this limit, the oldest version will be deleted after each new modification. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1ListDnsZoneVersionsResponse**](scaleway.domain.v2beta1.ListDNSZoneVersionsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_versions

> crate::models::ScalewayK8sV1ListVersionsResponse list_versions(region)
List all available versions

This method allows to list all available versions for the creation of a new Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |

### Return type

[**crate::models::ScalewayK8sV1ListVersionsResponse**](scaleway.k8s.v1.ListVersionsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_dns_zone_version

> serde_json::Value restore_dns_zone_version(dns_zone_version_id, body)
Restore DNS zone version

Restore and activate a previous DNS zone version.

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

