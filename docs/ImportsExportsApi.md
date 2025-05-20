# \ImportsExportsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_raw_dns_zone**](ImportsExportsApi.md#export_raw_dns_zone) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/raw | Export a raw DNS zone
[**import_provider_dns_zone**](ImportsExportsApi.md#import_provider_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/import-provider | Import a DNS zone from another provider
[**import_raw_dns_zone**](ImportsExportsApi.md#import_raw_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/raw | Import a raw DNS zone



## export_raw_dns_zone

> models::ScalewayPeriodStdPeriodFile export_raw_dns_zone(dns_zone, format)
Export a raw DNS zone

Export a DNS zone with default name servers, in a specific format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone to export. | [required] |
**format** | Option<**String**> | DNS zone format. |  |[default to bind]

### Return type

[**models::ScalewayPeriodStdPeriodFile**](scaleway.std.File.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_provider_dns_zone

> models::ScalewayPeriodDomainPeriodV2beta1PeriodImportProviderDnsZoneResponse import_provider_dns_zone(dns_zone, import_provider_dns_zone_request)
Import a DNS zone from another provider

Import and replace the format of records from a given provider, with default name servers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |
**import_provider_dns_zone_request** | [**ImportProviderDnsZoneRequest**](ImportProviderDnsZoneRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodImportProviderDnsZoneResponse**](scaleway.domain.v2beta1.ImportProviderDNSZoneResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_raw_dns_zone

> models::ScalewayPeriodDomainPeriodV2beta1PeriodImportRawDnsZoneResponse import_raw_dns_zone(dns_zone, import_raw_dns_zone_request)
Import a raw DNS zone

Import and replace the format of records from a given provider, with default name servers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone to import. | [required] |
**import_raw_dns_zone_request** | [**ImportRawDnsZoneRequest**](ImportRawDnsZoneRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodImportRawDnsZoneResponse**](scaleway.domain.v2beta1.ImportRawDNSZoneResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

