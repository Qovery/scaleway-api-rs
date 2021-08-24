# \ImportsExportsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_raw_dns_zone**](ImportsExportsApi.md#export_raw_dns_zone) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/raw | Export raw DNS zone
[**import_provider_dns_zone**](ImportsExportsApi.md#import_provider_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/import-provider | Import provider DNS zone
[**import_raw_dns_zone**](ImportsExportsApi.md#import_raw_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/raw | Import raw DNS zone



## export_raw_dns_zone

> crate::models::ScalewayStdFile export_raw_dns_zone(dns_zone, format)
Export raw DNS zone

Get a DNS zone in a given format with default NS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone to export | [required] |
**format** | Option<**String**> | Format for DNS zone |  |[default to bind]

### Return type

[**crate::models::ScalewayStdFile**](scaleway.std.File.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_provider_dns_zone

> crate::models::ScalewayDomainV2beta1ImportProviderDnsZoneResponse import_provider_dns_zone(dns_zone, inline_object49)
Import provider DNS zone

Import and replace records from a given provider format with default NS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |
**inline_object49** | [**InlineObject49**](InlineObject49.md) |  | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1ImportProviderDnsZoneResponse**](scaleway.domain.v2beta1.ImportProviderDNSZoneResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_raw_dns_zone

> crate::models::ScalewayDomainV2beta1ImportRawDnsZoneResponse import_raw_dns_zone(dns_zone, inline_object51)
Import raw DNS zone

Import and replace records from a given provider format with default NS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone to import | [required] |
**inline_object51** | [**InlineObject51**](InlineObject51.md) |  | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1ImportRawDnsZoneResponse**](scaleway.domain.v2beta1.ImportRawDNSZoneResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

