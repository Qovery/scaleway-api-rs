# \PrivilegesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_privileges**](PrivilegesApi.md#list_privileges) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/privileges | List user privileges for a database
[**set_privilege**](PrivilegesApi.md#set_privilege) | **PUT** /rdb/v1/regions/{region}/instances/{instance_id}/privileges | Set user privileges for a database



## list_privileges

> models::ScalewayPeriodRdbPeriodV1PeriodListPrivilegesResponse list_privileges(region, instance_id, order_by, page, page_size, database_name, user_name)
List user privileges for a database

List privileges of a user on a database. By default, the details returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field. You can define additional parameters for your query, such as `database_name` and `user_name`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance. | [required] |
**order_by** | Option<**String**> | Criteria to use when ordering privileges listing. |  |[default to user_name_asc]
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**database_name** | Option<**String**> | Name of the database. |  |
**user_name** | Option<**String**> | Name of the user. |  |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodListPrivilegesResponse**](scaleway.rdb.v1.ListPrivilegesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_privilege

> models::ScalewayPeriodRdbPeriodV1PeriodPrivilege set_privilege(region, instance_id, set_privilege_request)
Set user privileges for a database

Set the privileges of a user on a database. You must define `database_name`, `user_name` and `permission` in the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance. | [required] |
**set_privilege_request** | [**SetPrivilegeRequest**](SetPrivilegeRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodPrivilege**](scaleway.rdb.v1.Privilege.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

