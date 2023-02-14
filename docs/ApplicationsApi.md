# \ApplicationsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_application**](ApplicationsApi.md#create_application) | **POST** /iam/v1alpha1/applications | Create a new application
[**delete_application**](ApplicationsApi.md#delete_application) | **DELETE** /iam/v1alpha1/applications/{application_id} | Delete an application
[**get_application**](ApplicationsApi.md#get_application) | **GET** /iam/v1alpha1/applications/{application_id} | Get an existing application
[**list_applications**](ApplicationsApi.md#list_applications) | **GET** /iam/v1alpha1/applications | List applications of an organization
[**update_application**](ApplicationsApi.md#update_application) | **PATCH** /iam/v1alpha1/applications/{application_id} | Update an existing application



## create_application

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApplication create_application(create_application_request)
Create a new application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_application_request** | [**CreateApplicationRequest**](CreateApplicationRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApplication**](scaleway.iam.v1alpha1.Application.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application

> delete_application(application_id)
Delete an application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | ID of application to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApplication get_application(application_id)
Get an existing application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | ID of application to find | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApplication**](scaleway.iam.v1alpha1.Application.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_applications

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListApplicationsResponse list_applications(organization_id, order_by, page_size, page, name, editable, application_ids)
List applications of an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | ID of organization to filter | [required] |
**order_by** | Option<**String**> | Criteria for sorting results |  |[default to created_at_asc]
**page_size** | Option<**i32**> | Number of results per page. Value must be between 1 and 100 |  |
**page** | Option<**i32**> | Number of page. Value must be greater to 1 |  |
**name** | Option<**String**> | Name of application to filter |  |
**editable** | Option<**bool**> | Filter out editable applications or not |  |
**application_ids** | Option<[**Vec<String>**](String.md)> | Filter out by a list of ID |  |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListApplicationsResponse**](scaleway.iam.v1alpha1.ListApplicationsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApplication update_application(application_id, update_application_request)
Update an existing application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | ID of application to update | [required] |
**update_application_request** | [**UpdateApplicationRequest**](UpdateApplicationRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodApplication**](scaleway.iam.v1alpha1.Application.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

