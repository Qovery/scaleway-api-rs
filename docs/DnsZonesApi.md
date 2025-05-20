# \DnsZonesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_dns_zone**](DnsZonesApi.md#clone_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/clone | Clone a DNS zone
[**create_dns_zone**](DnsZonesApi.md#create_dns_zone) | **POST** /domain/v2beta1/dns-zones | Create a DNS zone
[**create_ssl_certificate**](DnsZonesApi.md#create_ssl_certificate) | **POST** /domain/v2beta1/ssl-certificates | Create or get the DNS zone's TLS certificate
[**delete_dns_zone**](DnsZonesApi.md#delete_dns_zone) | **DELETE** /domain/v2beta1/dns-zones/{dns_zone} | Delete a DNS zone
[**delete_dns_zone_tsig_key**](DnsZonesApi.md#delete_dns_zone_tsig_key) | **DELETE** /domain/v2beta1/dns-zones/{dns_zone}/tsig-key | Delete the DNS zone's TSIG key
[**delete_ssl_certificate**](DnsZonesApi.md#delete_ssl_certificate) | **DELETE** /domain/v2beta1/ssl-certificates/{dns_zone} | Delete a TLS certificate
[**get_dns_zone_tsig_key**](DnsZonesApi.md#get_dns_zone_tsig_key) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/tsig-key | Get the DNS zone's TSIG key
[**get_ssl_certificate**](DnsZonesApi.md#get_ssl_certificate) | **GET** /domain/v2beta1/ssl-certificates/{dns_zone} | Get a DNS zone's TLS certificate
[**list_dns_zones**](DnsZonesApi.md#list_dns_zones) | **GET** /domain/v2beta1/dns-zones | List DNS zones
[**list_ssl_certificates**](DnsZonesApi.md#list_ssl_certificates) | **GET** /domain/v2beta1/ssl-certificates | List a user's TLS certificates
[**refresh_dns_zone**](DnsZonesApi.md#refresh_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/refresh | Refresh a DNS zone
[**update_dns_zone**](DnsZonesApi.md#update_dns_zone) | **PATCH** /domain/v2beta1/dns-zones/{dns_zone} | Update a DNS zone



## clone_dns_zone

> models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone clone_dns_zone(dns_zone, clone_dns_zone_request)
Clone a DNS zone

Clone an existing DNS zone with all its records into a new DNS zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone to clone. | [required] |
**clone_dns_zone_request** | [**CloneDnsZoneRequest**](CloneDnsZoneRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone**](scaleway.domain.v2beta1.DNSZone.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dns_zone

> models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone create_dns_zone(create_dns_zone_request)
Create a DNS zone

Create a new DNS zone specified by the domain name, the subdomain and the Project ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dns_zone_request** | [**CreateDnsZoneRequest**](CreateDnsZoneRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone**](scaleway.domain.v2beta1.DNSZone.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ssl_certificate

> models::ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate create_ssl_certificate(create_ssl_certificate_request)
Create or get the DNS zone's TLS certificate

Create a new TLS certificate or retrieve information about an existing TLS certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_ssl_certificate_request** | [**CreateSslCertificateRequest**](CreateSslCertificateRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate**](scaleway.domain.v2beta1.SSLCertificate.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dns_zone

> serde_json::Value delete_dns_zone(dns_zone, project_id)
Delete a DNS zone

Delete a DNS zone and all its records.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone to delete. | [required] |
**project_id** | **String** | Project ID of the DNS zone to delete. | [required] |

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
Delete the DNS zone's TSIG key

Delete an existing TSIG key specified by its DNS zone. Deleting a TSIG key is permanent and cannot be undone.

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
Delete a TLS certificate

Delete an existing TLS certificate specified by its DNS zone. Deleting a TLS certificate is permanent and cannot be undone.

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

> models::ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneTsigKeyResponse get_dns_zone_tsig_key(dns_zone)
Get the DNS zone's TSIG key

Retrieve information about the TSIG key of a given DNS zone to allow AXFR requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneTsigKeyResponse**](scaleway.domain.v2beta1.GetDNSZoneTsigKeyResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ssl_certificate

> models::ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate get_ssl_certificate(dns_zone)
Get a DNS zone's TLS certificate

Get the DNS zone's TLS certificate. If you do not have a certificate, the ouptut returns `no certificate found`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate**](scaleway.domain.v2beta1.SSLCertificate.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_zones

> models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZonesResponse list_dns_zones(domain, organization_id, project_id, order_by, page, page_size, dns_zone, dns_zones, created_after, created_before, updated_after, updated_before)
List DNS zones

Retrieve the list of DNS zones you can manage and filter DNS zones associated with specific domain names.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | Domain on which to filter the returned DNS zones. | [required] |
**organization_id** | Option<**String**> | Organization ID on which to filter the returned DNS zones. |  |
**project_id** | Option<**String**> | Project ID on which to filter the returned DNS zones. |  |
**order_by** | Option<**String**> | Sort order of the returned DNS zones. |  |[default to domain_asc]
**page** | Option<**i32**> | Page number to return, from the paginated results. |  |
**page_size** | Option<**i32**> | Maximum number of DNS zones to return per page. |  |
**dns_zone** | Option<**String**> | DNS zone on which to filter the returned DNS zones. |  |
**dns_zones** | Option<[**Vec<String>**](String.md)> | DNS zones on which to filter the returned DNS zones. |  |
**created_after** | Option<**String**> | Only list DNS zones created after this date. (RFC 3339 format) |  |
**created_before** | Option<**String**> | Only list DNS zones created before this date. (RFC 3339 format) |  |
**updated_after** | Option<**String**> | Only list DNS zones updated after this date. (RFC 3339 format) |  |
**updated_before** | Option<**String**> | Only list DNS zones updated before this date. (RFC 3339 format) |  |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZonesResponse**](scaleway.domain.v2beta1.ListDNSZonesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ssl_certificates

> models::ScalewayPeriodDomainPeriodV2beta1PeriodListSslCertificatesResponse list_ssl_certificates(dns_zone, page, page_size, project_id)
List a user's TLS certificates

List all the TLS certificates a user has created, specified by the user's Project ID and the DNS zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**project_id** | Option<**String**> |  |  |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodListSslCertificatesResponse**](scaleway.domain.v2beta1.ListSSLCertificatesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_dns_zone

> models::ScalewayPeriodDomainPeriodV2beta1PeriodRefreshDnsZoneResponse refresh_dns_zone(dns_zone, refresh_dns_zone_request)
Refresh a DNS zone

Refresh an SOA DNS zone to reload the records in the DNS zone and update the SOA serial. You can recreate the given DNS zone and its sub DNS zone if needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone to refresh. | [required] |
**refresh_dns_zone_request** | [**RefreshDnsZoneRequest**](RefreshDnsZoneRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodRefreshDnsZoneResponse**](scaleway.domain.v2beta1.RefreshDNSZoneResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dns_zone

> models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone update_dns_zone(dns_zone, update_dns_zone_request)
Update a DNS zone

Update the name and/or the Organizations for a DNS zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone to update. | [required] |
**update_dns_zone_request** | [**UpdateDnsZoneRequest**](UpdateDnsZoneRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone**](scaleway.domain.v2beta1.DNSZone.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

