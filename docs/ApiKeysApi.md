# \ApiKeysApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](ApiKeysApi.md#create_api_key) | **POST** /iam/v1alpha1/api-keys | Create an API key
[**delete_api_key**](ApiKeysApi.md#delete_api_key) | **DELETE** /iam/v1alpha1/api-keys/{access_key} | Delete an API key
[**get_api_key**](ApiKeysApi.md#get_api_key) | **GET** /iam/v1alpha1/api-keys/{access_key} | Get an API key
[**list_api_keys**](ApiKeysApi.md#list_api_keys) | **GET** /iam/v1alpha1/api-keys | List API keys
[**update_api_key**](ApiKeysApi.md#update_api_key) | **PATCH** /iam/v1alpha1/api-keys/{access_key} | Update an API key



## create_api_key

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApiKey create_api_key(create_api_key_request)
Create an API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApiKey**](scaleway.iam.v1alpha1.APIKey.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> delete_api_key(access_key)
Delete an API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key** | **String** | Access key to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_key

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApiKey get_api_key(access_key)
Get an API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key** | **String** | Access key to search for | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApiKey**](scaleway.iam.v1alpha1.APIKey.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_api_keys

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListApiKeysResponse list_api_keys(organization_id, order_by, page, page_size, application_id, user_id, editable)
List API keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | ID of organization | [required] |
**order_by** | Option<**String**> | Criteria for sorting results |  |[default to created_at_asc]
**page** | Option<**i32**> | Number of page. Value must be greater or equals to 1 |  |
**page_size** | Option<**i32**> | Number of results per page. Value must be between 1 and 100 |  |
**application_id** | Option<**String**> | ID of an application bearer |  |
**user_id** | Option<**String**> | ID of a user bearer |  |
**editable** | Option<**bool**> | Filter out editable API keys or not |  |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListApiKeysResponse**](scaleway.iam.v1alpha1.ListAPIKeysResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApiKey update_api_key(access_key, update_api_key_request)
Update an API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key** | **String** | Access key to update | [required] |
**update_api_key_request** | [**UpdateApiKeyRequest**](UpdateApiKeyRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApiKey**](scaleway.iam.v1alpha1.APIKey.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

