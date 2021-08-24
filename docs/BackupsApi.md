# \BackupsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_database_backup**](BackupsApi.md#create_database_backup) | **POST** /rdb/v1/regions/{region}/backups | Create a database backup
[**delete_database_backup**](BackupsApi.md#delete_database_backup) | **DELETE** /rdb/v1/regions/{region}/backups/{database_backup_id} | Delete a database backup
[**export_database_backup**](BackupsApi.md#export_database_backup) | **POST** /rdb/v1/regions/{region}/backups/{database_backup_id}/export | Export a database backup
[**get_database_backup**](BackupsApi.md#get_database_backup) | **GET** /rdb/v1/regions/{region}/backups/{database_backup_id} | Get a database backup
[**list_database_backups**](BackupsApi.md#list_database_backups) | **GET** /rdb/v1/regions/{region}/backups | List database backups
[**restore_database_backup**](BackupsApi.md#restore_database_backup) | **POST** /rdb/v1/regions/{region}/backups/{database_backup_id}/restore | Restore a database backup
[**update_database_backup**](BackupsApi.md#update_database_backup) | **PATCH** /rdb/v1/regions/{region}/backups/{database_backup_id} | Update a database backup



## create_database_backup

> crate::models::ScalewayRdbV1DatabaseBackup create_database_backup(region, inline_object7)
Create a database backup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**inline_object7** | [**InlineObject7**](InlineObject7.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1DatabaseBackup**](scaleway.rdb.v1.DatabaseBackup.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_database_backup

> crate::models::ScalewayRdbV1DatabaseBackup delete_database_backup(region, database_backup_id)
Delete a database backup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**database_backup_id** | **String** | UUID of the database backup to delete | [required] |

### Return type

[**crate::models::ScalewayRdbV1DatabaseBackup**](scaleway.rdb.v1.DatabaseBackup.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_database_backup

> crate::models::ScalewayRdbV1DatabaseBackup export_database_backup(region, database_backup_id, body)
Export a database backup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**database_backup_id** | **String** | UUID of the database backup you want to export | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1DatabaseBackup**](scaleway.rdb.v1.DatabaseBackup.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_database_backup

> crate::models::ScalewayRdbV1DatabaseBackup get_database_backup(region, database_backup_id)
Get a database backup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**database_backup_id** | **String** | UUID of the database backup | [required] |

### Return type

[**crate::models::ScalewayRdbV1DatabaseBackup**](scaleway.rdb.v1.DatabaseBackup.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_database_backups

> crate::models::ScalewayRdbV1ListDatabaseBackupsResponse list_database_backups(region, name, order_by, instance_id, organization_id, project_id, page, page_size)
List database backups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**name** | Option<**String**> | Name of the database backups |  |
**order_by** | Option<**String**> | Criteria to use when ordering database backups listing |  |[default to created_at_asc]
**instance_id** | Option<**String**> | UUID of the instance |  |
**organization_id** | Option<**String**> | Organization ID the database backups belongs to |  |
**project_id** | Option<**String**> | Project ID the database backups belongs to |  |
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayRdbV1ListDatabaseBackupsResponse**](scaleway.rdb.v1.ListDatabaseBackupsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_database_backup

> crate::models::ScalewayRdbV1DatabaseBackup restore_database_backup(region, database_backup_id, inline_object9)
Restore a database backup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**database_backup_id** | **String** | Backup of a logical database | [required] |
**inline_object9** | [**InlineObject9**](InlineObject9.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1DatabaseBackup**](scaleway.rdb.v1.DatabaseBackup.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_database_backup

> crate::models::ScalewayRdbV1DatabaseBackup update_database_backup(region, database_backup_id, inline_object8)
Update a database backup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**database_backup_id** | **String** | UUID of the database backup to update | [required] |
**inline_object8** | [**InlineObject8**](InlineObject8.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1DatabaseBackup**](scaleway.rdb.v1.DatabaseBackup.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

