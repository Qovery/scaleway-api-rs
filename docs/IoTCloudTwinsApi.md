# \IoTCloudTwinsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_twin_document**](IoTCloudTwinsApi.md#delete_twin_document) | **DELETE** /iot/v1/regions/{region}/twins/{twin_id}/documents/{document_name} | BETA - Delete a Cloud Twin Document
[**delete_twin_documents**](IoTCloudTwinsApi.md#delete_twin_documents) | **DELETE** /iot/v1/regions/{region}/twins/{twin_id} | BETA - Delete all the documents of a Cloud Twin
[**get_twin_document**](IoTCloudTwinsApi.md#get_twin_document) | **GET** /iot/v1/regions/{region}/twins/{twin_id}/documents/{document_name} | BETA - Get a Cloud Twin Document
[**list_twin_documents**](IoTCloudTwinsApi.md#list_twin_documents) | **GET** /iot/v1/regions/{region}/twins/{twin_id} | BETA - List the documents of a Cloud Twin
[**patch_twin_document**](IoTCloudTwinsApi.md#patch_twin_document) | **PATCH** /iot/v1/regions/{region}/twins/{twin_id}/documents/{document_name} | BETA - Patch a Cloud Twin Document
[**put_twin_document**](IoTCloudTwinsApi.md#put_twin_document) | **PUT** /iot/v1/regions/{region}/twins/{twin_id}/documents/{document_name} | BETA - Update a Cloud Twin Document



## delete_twin_document

> delete_twin_document(region, twin_id, document_name)
BETA - Delete a Cloud Twin Document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**twin_id** | **String** | Twin ID | [required] |
**document_name** | **String** | Document name | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_twin_documents

> delete_twin_documents(region, twin_id)
BETA - Delete all the documents of a Cloud Twin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**twin_id** | **String** | Twin ID | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_twin_document

> crate::models::ScalewayIotV1TwinDocument get_twin_document(region, twin_id, document_name)
BETA - Get a Cloud Twin Document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**twin_id** | **String** | Twin ID | [required] |
**document_name** | **String** | Document name | [required] |

### Return type

[**crate::models::ScalewayIotV1TwinDocument**](scaleway.iot.v1.TwinDocument.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_twin_documents

> crate::models::ScalewayIotV1ListTwinDocumentsResponse list_twin_documents(region, twin_id)
BETA - List the documents of a Cloud Twin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**twin_id** | **String** | Twin ID | [required] |

### Return type

[**crate::models::ScalewayIotV1ListTwinDocumentsResponse**](scaleway.iot.v1.ListTwinDocumentsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_twin_document

> crate::models::ScalewayIotV1TwinDocument patch_twin_document(region, twin_id, document_name, patch_twin_document_request)
BETA - Patch a Cloud Twin Document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**twin_id** | **String** | Twin ID | [required] |
**document_name** | **String** | Document name | [required] |
**patch_twin_document_request** | [**PatchTwinDocumentRequest**](PatchTwinDocumentRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1TwinDocument**](scaleway.iot.v1.TwinDocument.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_twin_document

> crate::models::ScalewayIotV1TwinDocument put_twin_document(region, twin_id, document_name, put_twin_document_request)
BETA - Update a Cloud Twin Document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**twin_id** | **String** | Twin ID | [required] |
**document_name** | **String** | Document name | [required] |
**put_twin_document_request** | [**PutTwinDocumentRequest**](PutTwinDocumentRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayIotV1TwinDocument**](scaleway.iot.v1.TwinDocument.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

