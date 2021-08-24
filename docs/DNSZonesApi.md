# \DNSZonesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_dns_zone**](DNSZonesApi.md#clone_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/clone | Clone a DNS zone
[**create_dns_zone**](DNSZonesApi.md#create_dns_zone) | **POST** /domain/v2beta1/dns-zones | Create a DNS zone
[**create_ssl_certificate**](DNSZonesApi.md#create_ssl_certificate) | **POST** /domain/v2beta1/ssl-certificates | Create or return the zone TLS certificate
[**delete_dns_zone**](DNSZonesApi.md#delete_dns_zone) | **DELETE** /domain/v2beta1/dns-zones/{dns_zone} | Delete DNS zone
[**delete_dns_zone_tsig_key**](DNSZonesApi.md#delete_dns_zone_tsig_key) | **DELETE** /domain/v2beta1/dns-zones/{dns_zone}/tsig-key | Delete the DNS zone TSIG Key
[**delete_ssl_certificate**](DNSZonesApi.md#delete_ssl_certificate) | **DELETE** /domain/v2beta1/ssl-certificates/{dns_zone} | Delete an TLS certificate
[**get_dns_zone_tsig_key**](DNSZonesApi.md#get_dns_zone_tsig_key) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/tsig-key | Get the DNS zone TSIG Key
[**get_ssl_certificate**](DNSZonesApi.md#get_ssl_certificate) | **GET** /domain/v2beta1/ssl-certificates/{dns_zone} | Get the zone TLS certificate if it exists
[**list_dns_zones**](DNSZonesApi.md#list_dns_zones) | **GET** /domain/v2beta1/dns-zones | List DNS zones
[**list_ssl_certificates**](DNSZonesApi.md#list_ssl_certificates) | **GET** /domain/v2beta1/ssl-certificates | List all user TLS certificates
[**refresh_dns_zone**](DNSZonesApi.md#refresh_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/refresh | Refresh DNS zone
[**update_dns_zone**](DNSZonesApi.md#update_dns_zone) | **PATCH** /domain/v2beta1/dns-zones/{dns_zone} | Update a DNS zone



## clone_dns_zone

> crate::models::ScalewayDomainV2beta1DnsZone clone_dns_zone(dns_zone, inline_object48)
Clone a DNS zone

Clone an existed DNS zone with all its records into a new one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone to clone | [required] |
**inline_object48** | [**InlineObject48**](InlineObject48.md) |  | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1DnsZone**](scaleway.domain.v2beta1.DNSZone.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dns_zone

> crate::models::ScalewayDomainV2beta1DnsZone create_dns_zone(inline_object46)
Create a DNS zone

Create a new DNS zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object46** | [**InlineObject46**](InlineObject46.md) |  | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1DnsZone**](scaleway.domain.v2beta1.DNSZone.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ssl_certificate

> crate::models::ScalewayDomainV2beta1SslCertificate create_ssl_certificate(inline_object54)
Create or return the zone TLS certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object54** | [**InlineObject54**](InlineObject54.md) |  | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1SslCertificate**](scaleway.domain.v2beta1.SSLCertificate.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dns_zone

> serde_json::Value delete_dns_zone(dns_zone, project_id)
Delete DNS zone

Delete a DNS zone and all it's records.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone to delete | [required] |
**project_id** | **String** | The project ID of the DNS zone to delete | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dns_zone_tsig_key

> delete_dns_zone_tsig_key(dns_zone)
Delete the DNS zone TSIG Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ssl_certificate

> serde_json::Value delete_ssl_certificate(dns_zone)
Delete an TLS certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dns_zone_tsig_key

> crate::models::ScalewayDomainV2beta1GetDnsZoneTsigKeyResponse get_dns_zone_tsig_key(dns_zone)
Get the DNS zone TSIG Key

Get the DNS zone TSIG Key to allow AXFR request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1GetDnsZoneTsigKeyResponse**](scaleway.domain.v2beta1.GetDNSZoneTsigKeyResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ssl_certificate

> crate::models::ScalewayDomainV2beta1SslCertificate get_ssl_certificate(dns_zone)
Get the zone TLS certificate if it exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1SslCertificate**](scaleway.domain.v2beta1.SSLCertificate.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_zones

> crate::models::ScalewayDomainV2beta1ListDnsZonesResponse list_dns_zones(organization_id, project_id, order_by, page, page_size, domain, dns_zone)
List DNS zones

Returns a list of manageable DNS zones. You can filter the DNS zones by domain name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | The organization ID on which to filter the returned DNS zones |  |
**project_id** | Option<**String**> | The project ID on which to filter the returned DNS zones |  |
**order_by** | Option<**String**> | The sort order of the returned DNS zones |  |[default to domain_asc]
**page** | Option<**f32**> | The page number for the returned DNS zones |  |[default to 1]
**page_size** | Option<**f32**> | The maximum number of DNS zones per page |  |[default to 20]
**domain** | Option<**String**> | The domain on which to filter the returned DNS zones |  |
**dns_zone** | Option<**String**> | The DNS zone on which to filter the returned DNS zones |  |

### Return type

[**crate::models::ScalewayDomainV2beta1ListDnsZonesResponse**](scaleway.domain.v2beta1.ListDNSZonesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ssl_certificates

> crate::models::ScalewayDomainV2beta1ListSslCertificatesResponse list_ssl_certificates(dns_zone, page, page_size, project_id)
List all user TLS certificates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | Option<**String**> |  |  |
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | Page size |  |[default to 20]
**project_id** | Option<**String**> |  |  |

### Return type

[**crate::models::ScalewayDomainV2beta1ListSslCertificatesResponse**](scaleway.domain.v2beta1.ListSSLCertificatesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_dns_zone

> crate::models::ScalewayDomainV2beta1RefreshDnsZoneResponse refresh_dns_zone(dns_zone, inline_object53)
Refresh DNS zone

Refresh SOA DNS zone. You can recreate the given DNS zone and its sub DNS zone if needed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone to refresh | [required] |
**inline_object53** | [**InlineObject53**](InlineObject53.md) |  | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1RefreshDnsZoneResponse**](scaleway.domain.v2beta1.RefreshDNSZoneResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dns_zone

> crate::models::ScalewayDomainV2beta1DnsZone update_dns_zone(dns_zone, inline_object47)
Update a DNS zone

Update the name and/or the organizations for a DNS zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone to update | [required] |
**inline_object47** | [**InlineObject47**](InlineObject47.md) |  | [required] |

### Return type

[**crate::models::ScalewayDomainV2beta1DnsZone**](scaleway.domain.v2beta1.DNSZone.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

