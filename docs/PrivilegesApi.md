# \PrivilegesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_privileges**](PrivilegesApi.md#list_privileges) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/privileges | List privileges of a given user for a given database on a given instance
[**set_privilege**](PrivilegesApi.md#set_privilege) | **PUT** /rdb/v1/regions/{region}/instances/{instance_id}/privileges | Set privileges of a given user for a given database on a given instance



## list_privileges

> crate::models::ScalewayRdbV1ListPrivilegesResponse list_privileges(region, instance_id, order_by, page, page_size, database_name, user_name)
List privileges of a given user for a given database on a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**order_by** | Option<**String**> | Criteria to use when ordering privileges listing |  |[default to user_name_asc]
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | Page size |  |[default to 20]
**database_name** | Option<**String**> | Name of the database |  |
**user_name** | Option<**String**> | Name of the user |  |

### Return type

[**crate::models::ScalewayRdbV1ListPrivilegesResponse**](scaleway.rdb.v1.ListPrivilegesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_privilege

> crate::models::ScalewayRdbV1Privilege set_privilege(region, instance_id, inline_object18)
Set privileges of a given user for a given database on a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**inline_object18** | [**InlineObject18**](InlineObject18.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Privilege**](scaleway.rdb.v1.Privilege.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

