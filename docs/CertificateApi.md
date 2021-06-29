# \CertificateApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_certificate**](CertificateApi.md#create_certificate) | **post** /lb/v1/regions/{region}/lbs/{lb_id}/certificates | Create a TLS certificate
[**delete_certificate**](CertificateApi.md#delete_certificate) | **delete** /lb/v1/regions/{region}/certificates/{certificate_id} | Delete a TLS certificate
[**get_certificate**](CertificateApi.md#get_certificate) | **get** /lb/v1/regions/{region}/certificates/{certificate_id} | Get a TLS certificate
[**list_certificates**](CertificateApi.md#list_certificates) | **get** /lb/v1/regions/{region}/lbs/{lb_id}/certificates | List all TLS certificates on a given load balancer
[**update_certificate**](CertificateApi.md#update_certificate) | **put** /lb/v1/regions/{region}/certificates/{certificate_id} | Update a TLS certificate



## create_certificate

> crate::models::ScalewayLbV1Certificate create_certificate(region, lb_id, inline_object44)
Create a TLS certificate

Generate a new TLS certificate using Let's Encrypt or import your certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**inline_object44** | [**InlineObject44**](InlineObject44.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Certificate**](scaleway.lb.v1.Certificate.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_certificate

> delete_certificate(region, certificate_id)
Delete a TLS certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**certificate_id** | **String** | Certificate ID | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificate

> crate::models::ScalewayLbV1Certificate get_certificate(region, certificate_id)
Get a TLS certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**certificate_id** | **String** | Certificate ID | [required] |

### Return type

[**crate::models::ScalewayLbV1Certificate**](scaleway.lb.v1.Certificate.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_certificates

> crate::models::ScalewayLbV1ListCertificatesResponse list_certificates(region, lb_id, order_by, page, page_size, name)
List all TLS certificates on a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**order_by** | Option<**String**> | You can order the response by created_at asc/desc or name asc/desc |  |[default to created_at_asc]
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | The number of items to return |  |[default to 20]
**name** | Option<**String**> | Use this to search by name |  |

### Return type

[**crate::models::ScalewayLbV1ListCertificatesResponse**](scaleway.lb.v1.ListCertificatesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_certificate

> crate::models::ScalewayLbV1Certificate update_certificate(region, certificate_id, inline_object35)
Update a TLS certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**certificate_id** | **String** | Certificate ID | [required] |
**inline_object35** | [**InlineObject35**](InlineObject35.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Certificate**](scaleway.lb.v1.Certificate.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

