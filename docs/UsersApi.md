# \UsersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/users | Create a user on a given instance
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/users/{name} | Delete a user on a given instance
[**list_users**](UsersApi.md#list_users) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/users | List users of a given instance
[**update_user**](UsersApi.md#update_user) | **PATCH** /rdb/v1/regions/{region}/instances/{instance_id}/users/{name} | Update a user on a given instance



## create_user

> crate::models::ScalewayRdbV1User create_user(region, instance_id, inline_object24)
Create a user on a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to create a user in | [required] |
**inline_object24** | [**InlineObject24**](InlineObject24.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1User**](scaleway.rdb.v1.User.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(region, instance_id, name)
Delete a user on a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance to delete a user from | [required] |
**name** | **String** | Name of the user | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> crate::models::ScalewayRdbV1ListUsersResponse list_users(region, instance_id, name, order_by, page, page_size)
List users of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**name** | Option<**String**> | Name of the user |  |
**order_by** | Option<**String**> | Criteria to use when ordering users listing |  |[default to name_asc]
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayRdbV1ListUsersResponse**](scaleway.rdb.v1.ListUsersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::ScalewayRdbV1User update_user(region, instance_id, name, inline_object25)
Update a user on a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance the user belongs to | [required] |
**name** | **String** | Name of the database user | [required] |
**inline_object25** | [**InlineObject25**](InlineObject25.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1User**](scaleway.rdb.v1.User.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

