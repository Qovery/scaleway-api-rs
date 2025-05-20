# \SnapshotsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_instance_from_snapshot**](SnapshotsApi.md#create_instance_from_snapshot) | **POST** /rdb/v1/regions/{region}/snapshots/{snapshot_id}/create-instance | Create a new Database Instance from a snapshot
[**create_snapshot**](SnapshotsApi.md#create_snapshot) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/snapshots | Create a Database Instance snapshot
[**delete_snapshot**](SnapshotsApi.md#delete_snapshot) | **DELETE** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Delete a Database Instance snapshot
[**get_snapshot**](SnapshotsApi.md#get_snapshot) | **GET** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Get a Database Instance snapshot
[**list_snapshots**](SnapshotsApi.md#list_snapshots) | **GET** /rdb/v1/regions/{region}/snapshots | List snapshots
[**update_snapshot**](SnapshotsApi.md#update_snapshot) | **PATCH** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Update a Database Instance snapshot



## create_instance_from_snapshot

> models::ScalewayPeriodRdbPeriodV1PeriodInstance create_instance_from_snapshot(region, snapshot_id, create_instance_from_snapshot_request)
Create a new Database Instance from a snapshot

Restore a snapshot. When you restore a snapshot, a new Instance is created and billed to your account. Note that is possible to select a larger node type for your new Database Instance. However, the Block volume size will be the same as the size of the restored snapshot. All Instance settings will be restored if you chose a node type with the same or more memory size than the initial Instance. Settings will be reset to the default if your node type has less memory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | Block snapshot of the Database Instance. | [required] |
**create_instance_from_snapshot_request** | [**CreateInstanceFromSnapshotRequest**](CreateInstanceFromSnapshotRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_snapshot

> models::ScalewayPeriodRdbPeriodV1PeriodSnapshot create_snapshot(region, instance_id, create_snapshot_request)
Create a Database Instance snapshot

Create a new snapshot of a Database Instance. You must define the `name` parameter in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance. | [required] |
**create_snapshot_request** | [**CreateSnapshotRequest**](CreateSnapshotRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodSnapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snapshot

> models::ScalewayPeriodRdbPeriodV1PeriodSnapshot delete_snapshot(region, snapshot_id)
Delete a Database Instance snapshot

Delete a given snapshot of a Database Instance. You must specify, in the endpoint,  the `region` and `snapshot_id` parameters of the snapshot you want to delete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot to delete. | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodSnapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshot

> models::ScalewayPeriodRdbPeriodV1PeriodSnapshot get_snapshot(region, snapshot_id)
Get a Database Instance snapshot

Retrieve information about a given snapshot, specified by its `snapshot_id` and `region`. Full details about the snapshot, like size and expiration date, are returned in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot. | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodSnapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_snapshots

> models::ScalewayPeriodRdbPeriodV1PeriodListSnapshotsResponse list_snapshots(region, name, order_by, instance_id, organization_id, project_id, page, page_size)
List snapshots

List snapshots. You can include the `instance_id` or `project_id` in your query to get the list of snapshots for specific Database Instances and/or Projects. By default, the details returned in the list are ordered by creation date in ascending order, though this can be modified via the `order_by` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**name** | Option<**String**> | Name of the snapshot. |  |
**order_by** | Option<**String**> | Criteria to use when ordering snapshot listing. |  |[default to created_at_asc]
**instance_id** | Option<**String**> | UUID of the Database Instance. |  |
**organization_id** | Option<**String**> | Organization ID the snapshots belongs to. |  |
**project_id** | Option<**String**> | Project ID the snapshots belongs to. |  |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodListSnapshotsResponse**](scaleway.rdb.v1.ListSnapshotsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_snapshot

> models::ScalewayPeriodRdbPeriodV1PeriodSnapshot update_snapshot(region, snapshot_id, update_snapshot_request)
Update a Database Instance snapshot

Update the parameters of a snapshot of a Database Instance. You can update the `name` and `expires_at` parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot to update. | [required] |
**update_snapshot_request** | [**UpdateSnapshotRequest**](UpdateSnapshotRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodSnapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

