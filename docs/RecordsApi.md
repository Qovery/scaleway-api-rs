# \RecordsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_dns_zone_records**](RecordsApi.md#clear_dns_zone_records) | **DELETE** /domain/v2beta1/dns-zones/{dns_zone}/records | Clear records within a DNS zone
[**list_dns_zone_nameservers**](RecordsApi.md#list_dns_zone_nameservers) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/nameservers | List name servers within a DNS zone
[**list_dns_zone_records**](RecordsApi.md#list_dns_zone_records) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/records | List records within a DNS zone
[**update_dns_zone_nameservers**](RecordsApi.md#update_dns_zone_nameservers) | **PUT** /domain/v2beta1/dns-zones/{dns_zone}/nameservers | Update name servers within a DNS zone
[**update_dns_zone_records**](RecordsApi.md#update_dns_zone_records) | **PATCH** /domain/v2beta1/dns-zones/{dns_zone}/records | Update records within a DNS zone



## clear_dns_zone_records

> serde_json::Value clear_dns_zone_records(dns_zone)
Clear records within a DNS zone

Delete all records within a DNS zone that has default name servers.<br/> All edits will be versioned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone to clear. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_zone_nameservers

> models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneNameserversResponse list_dns_zone_nameservers(dns_zone, project_id)
List name servers within a DNS zone

Retrieve a list of name servers within a DNS zone and their optional glue records.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone on which to filter the returned DNS zone name servers. | [required] |
**project_id** | Option<**String**> | Project ID on which to filter the returned DNS zone name servers. |  |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneNameserversResponse**](scaleway.domain.v2beta1.ListDNSZoneNameserversResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_zone_records

> models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneRecordsResponse list_dns_zone_records(dns_zone, name, project_id, order_by, page, page_size, r#type, id)
List records within a DNS zone

Retrieve a list of DNS records within a DNS zone that has default name servers. You can filter records by type and name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone on which to filter the returned DNS zone records. | [required] |
**name** | **String** | Name on which to filter the returned DNS zone records. | [required] |
**project_id** | Option<**String**> | Project ID on which to filter the returned DNS zone records. |  |
**order_by** | Option<**String**> | Sort order of the returned DNS zone records. |  |[default to name_asc]
**page** | Option<**i32**> | Page number to return, from the paginated results. |  |
**page_size** | Option<**i32**> | Maximum number of DNS zone records per page. |  |
**r#type** | Option<**String**> | Record type on which to filter the returned DNS zone records. |  |[default to unknown]
**id** | Option<**String**> | Record ID on which to filter the returned DNS zone records. |  |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneRecordsResponse**](scaleway.domain.v2beta1.ListDNSZoneRecordsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dns_zone_nameservers

> models::ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneNameserversResponse update_dns_zone_nameservers(dns_zone, update_dns_zone_nameservers_request)
Update name servers within a DNS zone

Update name servers within a DNS zone and set optional glue records.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone in which to update the DNS zone name servers. | [required] |
**update_dns_zone_nameservers_request** | [**UpdateDnsZoneNameserversRequest**](UpdateDnsZoneNameserversRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneNameserversResponse**](scaleway.domain.v2beta1.UpdateDNSZoneNameserversResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dns_zone_records

> models::ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneRecordsResponse update_dns_zone_records(dns_zone, update_dns_zone_records_request)
Update records within a DNS zone

Update records within a DNS zone that has default name servers and perform several actions on your records.  Actions include:  - add: allows you to add a new record or add a new IP to an existing A record, for example  - set: allows you to edit a record or edit an IP from an existing A record, for example  - delete: allows you to delete a record or delete an IP from an existing A record, for example  - clear: allows you to delete all records from a DNS zone  All edits will be versioned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | DNS zone in which to update the DNS zone records. | [required] |
**update_dns_zone_records_request** | [**UpdateDnsZoneRecordsRequest**](UpdateDnsZoneRecordsRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneRecordsResponse**](scaleway.domain.v2beta1.UpdateDNSZoneRecordsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

