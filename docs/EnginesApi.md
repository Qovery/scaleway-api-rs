# \EnginesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_database_engines**](EnginesApi.md#list_database_engines) | **GET** /rdb/v1/regions/{region}/database-engines | List available database engines



## list_database_engines

> crate::models::ScalewayPeriodRdbPeriodV1PeriodListDatabaseEnginesResponse list_database_engines(region, name, version, page, page_size)
List available database engines

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**name** | Option<**String**> | Name of the Database Engine |  |
**version** | Option<**String**> | Version of the Database Engine |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodListDatabaseEnginesResponse**](scaleway.rdb.v1.ListDatabaseEnginesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

