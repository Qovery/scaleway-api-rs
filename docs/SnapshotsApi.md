# \SnapshotsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_instance_from_snapshot**](SnapshotsApi.md#create_instance_from_snapshot) | **POST** /rdb/v1/regions/{region}/snapshots/{snapshot_id}/create-instance | Create a new instance from a given snapshot
[**create_snapshot**](SnapshotsApi.md#create_snapshot) | **POST** /instance/v1/zones/{zone}/snapshots | Create a snapshot from a given volume or from a QCOW2 file
[**create_snapshot1**](SnapshotsApi.md#create_snapshot1) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/snapshots | Create an instance snapshot
[**delete_snapshot**](SnapshotsApi.md#delete_snapshot) | **DELETE** /instance/v1/zones/{zone}/snapshots/{snapshot_id} | Delete a snapshot
[**delete_snapshot1**](SnapshotsApi.md#delete_snapshot1) | **DELETE** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Delete an instance snapshot
[**export_snapshot**](SnapshotsApi.md#export_snapshot) | **POST** /instance/v1/zones/{zone}/snapshots/{snapshot_id}/export | Export a snapshot
[**get_snapshot**](SnapshotsApi.md#get_snapshot) | **GET** /instance/v1/zones/{zone}/snapshots/{snapshot_id} | Get a snapshot
[**get_snapshot1**](SnapshotsApi.md#get_snapshot1) | **GET** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Get an instance snapshot
[**list_snapshots**](SnapshotsApi.md#list_snapshots) | **GET** /instance/v1/zones/{zone}/snapshots | List snapshots
[**list_snapshots1**](SnapshotsApi.md#list_snapshots1) | **GET** /rdb/v1/regions/{region}/snapshots | List instance snapshots
[**set_snapshot**](SnapshotsApi.md#set_snapshot) | **PUT** /instance/v1/zones/{zone}/snapshots/{snapshot_id} | Update snapshot
[**update_snapshot**](SnapshotsApi.md#update_snapshot) | **PATCH** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Update an instance snapshot



## create_instance_from_snapshot

> crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance create_instance_from_snapshot(region, snapshot_id, create_instance_from_snapshot_request)
Create a new instance from a given snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | Block snapshot of the instance | [required] |
**create_instance_from_snapshot_request** | [**CreateInstanceFromSnapshotRequest**](CreateInstanceFromSnapshotRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_snapshot

> crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateSnapshotResponse create_snapshot(zone, create_snapshot_request)
Create a snapshot from a given volume or from a QCOW2 file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_snapshot_request** | [**CreateSnapshotRequest**](CreateSnapshotRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateSnapshotResponse**](scaleway.instance.v1.CreateSnapshotResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_snapshot1

> crate::models::ScalewayPeriodRdbPeriodV1PeriodSnapshot create_snapshot1(region, instance_id, create_snapshot1_request)
Create an instance snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**create_snapshot1_request** | [**CreateSnapshot1Request**](CreateSnapshot1Request.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodSnapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snapshot

> delete_snapshot(zone, snapshot_id)
Delete a snapshot

Delete the snapshot with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot you want to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snapshot1

> crate::models::ScalewayPeriodRdbPeriodV1PeriodSnapshot delete_snapshot1(region, snapshot_id)
Delete an instance snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot to delete | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodSnapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_snapshot

> crate::models::ScalewayPeriodInstancePeriodV1PeriodExportSnapshotResponse export_snapshot(zone, snapshot_id, export_snapshot_request)
Export a snapshot

Export a snapshot to a given S3 bucket in the same region.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**snapshot_id** | **String** | The snapshot ID | [required] |
**export_snapshot_request** | [**ExportSnapshotRequest**](ExportSnapshotRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodExportSnapshotResponse**](scaleway.instance.v1.ExportSnapshotResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshot

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetSnapshotResponse get_snapshot(zone, snapshot_id)
Get a snapshot

Get details of a snapshot with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot you want to get | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetSnapshotResponse**](scaleway.instance.v1.GetSnapshotResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshot1

> crate::models::ScalewayPeriodRdbPeriodV1PeriodSnapshot get_snapshot1(region, snapshot_id)
Get an instance snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodSnapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_snapshots

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListSnapshotsResponse list_snapshots(zone, organization, per_page, page, name, project, tags)
List snapshots

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**organization** | Option<**String**> |  |  |
**per_page** | Option<**i32**> |  |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**name** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |
**tags** | Option<**String**> |  |  |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListSnapshotsResponse**](scaleway.instance.v1.ListSnapshotsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_snapshots1

> crate::models::ScalewayPeriodRdbPeriodV1PeriodListSnapshotsResponse list_snapshots1(region, name, order_by, instance_id, organization_id, project_id, page, page_size)
List instance snapshots

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**name** | Option<**String**> | Name of the snapshot |  |
**order_by** | Option<**String**> | Criteria to use when ordering snapshot listing |  |[default to created_at_asc]
**instance_id** | Option<**String**> | UUID of the instance |  |
**organization_id** | Option<**String**> | Organization ID the snapshots belongs to |  |
**project_id** | Option<**String**> | Project ID the snapshots belongs to |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodListSnapshotsResponse**](scaleway.rdb.v1.ListSnapshotsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_snapshot

> crate::models::ScalewayPeriodInstancePeriodV1PeriodSetSnapshotResponse set_snapshot(zone, snapshot_id, set_snapshot_request)
Update snapshot

Replace all snapshot properties with a snapshot message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**snapshot_id** | **String** |  | [required] |
**set_snapshot_request** | [**SetSnapshotRequest**](SetSnapshotRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodSetSnapshotResponse**](scaleway.instance.v1.SetSnapshotResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_snapshot

> crate::models::ScalewayPeriodRdbPeriodV1PeriodSnapshot update_snapshot(region, snapshot_id, update_snapshot_request)
Update an instance snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot to update | [required] |
**update_snapshot_request** | [**UpdateSnapshotRequest**](UpdateSnapshotRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodSnapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

