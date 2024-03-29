# \DatabasesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_database**](DatabasesApi.md#create_database) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/databases | Create a database in a given instance
[**delete_database**](DatabasesApi.md#delete_database) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/databases/{name} | Delete a database in a given instance
[**list_databases**](DatabasesApi.md#list_databases) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/databases | List all database in a given instance



## create_database

> crate::models::ScalewayPeriodRdbPeriodV1PeriodDatabase create_database(region, instance_id, create_database_request)
Create a database in a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance where to create the database | [required] |
**create_database_request** | [**CreateDatabaseRequest**](CreateDatabaseRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodDatabase**](scaleway.rdb.v1.Database.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_database

> delete_database(region, instance_id, name)
Delete a database in a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance where to delete the database | [required] |
**name** | **String** | Name of the database to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_databases

> crate::models::ScalewayPeriodRdbPeriodV1PeriodListDatabasesResponse list_databases(region, instance_id, name, managed, owner, order_by, page, page_size)
List all database in a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance to list database of | [required] |
**name** | Option<**String**> | Name of the database |  |
**managed** | Option<**bool**> | Whether or not the database is managed |  |
**owner** | Option<**String**> | User that owns this database |  |
**order_by** | Option<**String**> | Criteria to use when ordering database listing |  |[default to name_asc]
**page** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodListDatabasesResponse**](scaleway.rdb.v1.ListDatabasesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

