# \DatabasesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_database**](DatabasesApi.md#create_database) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/databases | Create a database in a Database Instance
[**delete_database**](DatabasesApi.md#delete_database) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/databases/{name} | Delete a database in a Database Instance
[**list_databases**](DatabasesApi.md#list_databases) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/databases | List databases in a Database Instance



## create_database

> models::ScalewayPeriodRdbPeriodV1PeriodDatabase create_database(region, instance_id, create_database_request)
Create a database in a Database Instance

Create a new database. You must define the `name` parameter in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance where to create the database. | [required] |
**create_database_request** | [**CreateDatabaseRequest**](CreateDatabaseRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodDatabase**](scaleway.rdb.v1.Database.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_database

> delete_database(region, instance_id, name)
Delete a database in a Database Instance

Delete a given database on a Database Instance. You must specify, in the endpoint, the `region`, `instance_id` and `name` parameters of the database you want to delete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance where to delete the database. | [required] |
**name** | **String** | Name of the database to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_databases

> models::ScalewayPeriodRdbPeriodV1PeriodListDatabasesResponse list_databases(region, instance_id, name, managed, owner, order_by, page, page_size)
List databases in a Database Instance

List all databases of a given Database Instance. By default, the databases returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field. You can define additional parameters for your query, such as `name`, `managed` and `owner`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance to list the databases of. | [required] |
**name** | Option<**String**> | Name of the database. |  |
**managed** | Option<**bool**> | Defines whether or not the database is managed. |  |
**owner** | Option<**String**> | User that owns this database. |  |
**order_by** | Option<**String**> | Criteria to use when ordering database listing. |  |[default to name_asc]
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodListDatabasesResponse**](scaleway.rdb.v1.ListDatabasesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

