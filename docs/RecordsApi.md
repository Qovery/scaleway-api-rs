# \RecordsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_dns_zone_records**](RecordsApi.md#clear_dns_zone_records) | **DELETE** /domain/v2beta1/dns-zones/{dns_zone}/records | Clear DNS zone records
[**list_dns_zone_nameservers**](RecordsApi.md#list_dns_zone_nameservers) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/nameservers | List DNS zone nameservers
[**list_dns_zone_records**](RecordsApi.md#list_dns_zone_records) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/records | List DNS zone records
[**update_dns_zone_nameservers**](RecordsApi.md#update_dns_zone_nameservers) | **PUT** /domain/v2beta1/dns-zones/{dns_zone}/nameservers | Update DNS zone nameservers
[**update_dns_zone_records**](RecordsApi.md#update_dns_zone_records) | **PATCH** /domain/v2beta1/dns-zones/{dns_zone}/records | Update DNS zone records



## clear_dns_zone_records

> serde_json::Value clear_dns_zone_records(dns_zone)
Clear DNS zone records

Only available with default NS.<br/> Delete all the records from a DNS zone. All edits will be versioned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone to clear | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_zone_nameservers

> crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneNameserversResponse list_dns_zone_nameservers(dns_zone, project_id)
List DNS zone nameservers

Returns a list of Nameservers and their optional glue records for a DNS zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone on which to filter the returned DNS zone nameservers | [required] |
**project_id** | Option<**String**> | The project ID on which to filter the returned DNS zone nameservers |  |

### Return type

[**crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneNameserversResponse**](scaleway.domain.v2beta1.ListDNSZoneNameserversResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_zone_records

> crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneRecordsResponse list_dns_zone_records(dns_zone, project_id, order_by, page, page_size, name, r#type, id)
List DNS zone records

Returns a list of DNS records of a DNS zone with default NS. You can filter the records by type and name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone on which to filter the returned DNS zone records | [required] |
**project_id** | Option<**String**> | The project ID on which to filter the returned DNS zone records |  |
**order_by** | Option<**String**> | The sort order of the returned DNS zone records |  |[default to name_asc]
**page** | Option<**i32**> | The page number for the returned DNS zone records |  |[default to 1]
**page_size** | Option<**i32**> | The maximum number of DNS zone records per page |  |[default to 20]
**name** | Option<**String**> | The name on which to filter the returned DNS zone records |  |
**r#type** | Option<**String**> | The record type on which to filter the returned DNS zone records |  |[default to unknown]
**id** | Option<**String**> | The record ID on which to filter the returned DNS zone records |  |

### Return type

[**crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneRecordsResponse**](scaleway.domain.v2beta1.ListDNSZoneRecordsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dns_zone_nameservers

> crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneNameserversResponse update_dns_zone_nameservers(dns_zone, update_dns_zone_nameservers_request)
Update DNS zone nameservers

Update DNS zone nameservers and set optional glue records.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone where the DNS zone nameservers will be updated | [required] |
**update_dns_zone_nameservers_request** | [**UpdateDnsZoneNameserversRequest**](UpdateDnsZoneNameserversRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneNameserversResponse**](scaleway.domain.v2beta1.UpdateDNSZoneNameserversResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dns_zone_records

> crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneRecordsResponse update_dns_zone_records(dns_zone, update_dns_zone_records_request)
Update DNS zone records

Only available with default NS.<br/> Send a list of actions and records.  Action can be:  - add:   - Add new record   - Can be more specific and add a new IP to an existing A record for example  - set:   - Edit a record   - Can be more specific and edit an IP from an existing A record for example  - delete:   - Delete a record   - Can be more specific and delete an IP from an existing A record for example  - clear:   - Delete all records from a DNS zone  All edits will be versioned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone** | **String** | The DNS zone where the DNS zone records will be updated | [required] |
**update_dns_zone_records_request** | [**UpdateDnsZoneRecordsRequest**](UpdateDnsZoneRecordsRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneRecordsResponse**](scaleway.domain.v2beta1.UpdateDNSZoneRecordsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

