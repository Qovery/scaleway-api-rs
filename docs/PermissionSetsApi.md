# \PermissionSetsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_permission_sets**](PermissionSetsApi.md#list_permission_sets) | **GET** /iam/v1alpha1/permission-sets | List permission sets



## list_permission_sets

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListPermissionSetsResponse list_permission_sets(organization_id, order_by, page_size, page)
List permission sets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | Filter by organization ID | [required] |
**order_by** | Option<**String**> | Criteria for sorting results |  |[default to created_at_asc]
**page_size** | Option<**i32**> | Number of results per page. Value must be between 1 and 100 |  |
**page** | Option<**i32**> | Number of page. Value must be greater to 1 |  |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListPermissionSetsResponse**](scaleway.iam.v1alpha1.ListPermissionSetsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

