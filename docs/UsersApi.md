# \UsersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/users | Create a user on a given instance
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /iam/v1alpha1/users/{user_id} | Delete a guest user from an organization
[**delete_user1**](UsersApi.md#delete_user1) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/users/{name} | Delete a user on a given instance
[**get_user**](UsersApi.md#get_user) | **GET** /iam/v1alpha1/users/{user_id} | Retrieve a user from its ID
[**list_users**](UsersApi.md#list_users) | **GET** /iam/v1alpha1/users | List users of an organization
[**list_users1**](UsersApi.md#list_users1) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/users | List users of a given instance
[**update_user**](UsersApi.md#update_user) | **PATCH** /rdb/v1/regions/{region}/instances/{instance_id}/users/{name} | Update a user on a given instance



## create_user

> crate::models::ScalewayPeriodRdbPeriodV1PeriodUser create_user(region, instance_id, create_user_request)
Create a user on a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to create a user in | [required] |
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodUser**](scaleway.rdb.v1.User.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(user_id)
Delete a guest user from an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of user to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user1

> delete_user1(region, instance_id, name)
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


## get_user

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodUser get_user(user_id)
Retrieve a user from its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of user to find | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodUser**](scaleway.iam.v1alpha1.User.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListUsersResponse list_users(organization_id, order_by, page_size, page, user_ids)
List users of an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | ID of organization to filter | [required] |
**order_by** | Option<**String**> | Criteria for sorting results |  |[default to created_at_asc]
**page_size** | Option<**i32**> | Number of results per page. Value must be between 1 and 100 |  |
**page** | Option<**i32**> | Number of page. Value must be greater or equals to 1 |  |
**user_ids** | Option<[**Vec<String>**](String.md)> | Filter out by a list of ID |  |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListUsersResponse**](scaleway.iam.v1alpha1.ListUsersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users1

> crate::models::ScalewayPeriodRdbPeriodV1PeriodListUsersResponse list_users1(region, instance_id, name, order_by, page, page_size)
List users of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**name** | Option<**String**> | Name of the user |  |
**order_by** | Option<**String**> | Criteria to use when ordering users listing |  |[default to name_asc]
**page** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodListUsersResponse**](scaleway.rdb.v1.ListUsersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::ScalewayPeriodRdbPeriodV1PeriodUser update_user(region, instance_id, name, update_user_request)
Update a user on a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance the user belongs to | [required] |
**name** | **String** | Name of the database user | [required] |
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodUser**](scaleway.rdb.v1.User.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

