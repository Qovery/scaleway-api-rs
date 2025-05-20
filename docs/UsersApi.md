# \UsersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/users | Create a user for a Database Instance
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/users/{name} | Delete a user on a Database Instance
[**list_users**](UsersApi.md#list_users) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/users | List users of a Database Instance
[**update_user**](UsersApi.md#update_user) | **PATCH** /rdb/v1/regions/{region}/instances/{instance_id}/users/{name} | Update a user on a Database Instance



## create_user

> models::ScalewayPeriodRdbPeriodV1PeriodUser create_user(region, instance_id, create_user_request)
Create a user for a Database Instance

Create a new user for a Database Instance. You must define the `name`, `password` and `is_admin` parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance in which you want to create a user. | [required] |
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodUser**](scaleway.rdb.v1.User.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(region, instance_id, name)
Delete a user on a Database Instance

Delete a given user on a Database Instance. You must specify, in the endpoint,  the `region`, `instance_id` and `name` parameters of the user you want to delete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance to delete the user from. | [required] |
**name** | **String** | Name of the user. | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> models::ScalewayPeriodRdbPeriodV1PeriodListUsersResponse list_users(region, instance_id, name, order_by, page, page_size)
List users of a Database Instance

List all users of a given Database Instance. By default, the users returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance. | [required] |
**name** | Option<**String**> | Name of the user. |  |
**order_by** | Option<**String**> | Criteria to use when requesting user listing. |  |[default to name_asc]
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodListUsersResponse**](scaleway.rdb.v1.ListUsersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::ScalewayPeriodRdbPeriodV1PeriodUser update_user(region, instance_id, name, update_user_request)
Update a user on a Database Instance

Update the parameters of a user on a Database Instance. You can update the `password` and `is_admin` parameters, but you cannot change the name of the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance the user belongs to. | [required] |
**name** | **String** | Name of the database user. | [required] |
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodUser**](scaleway.rdb.v1.User.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

