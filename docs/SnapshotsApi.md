# \SnapshotsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_instance_from_snapshot**](SnapshotsApi.md#create_instance_from_snapshot) | **POST** /rdb/v1/regions/{region}/snapshots/{snapshot_id}/create-instance | Create a new instance from a given snapshot
[**create_snapshot**](SnapshotsApi.md#create_snapshot) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/snapshots | Create an instance snapshot
[**delete_snapshot**](SnapshotsApi.md#delete_snapshot) | **DELETE** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Delete an instance snapshot
[**get_snapshot**](SnapshotsApi.md#get_snapshot) | **GET** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Get an instance snapshot
[**list_snapshots**](SnapshotsApi.md#list_snapshots) | **GET** /rdb/v1/regions/{region}/snapshots | List instance snapshots
[**update_snapshot**](SnapshotsApi.md#update_snapshot) | **PATCH** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Update an instance snapshot



## create_instance_from_snapshot

> crate::models::ScalewayRdbV1Instance create_instance_from_snapshot(region, snapshot_id, inline_object27)
Create a new instance from a given snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | Block snapshot of the instance | [required] |
**inline_object27** | [**InlineObject27**](InlineObject27.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Instance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_snapshot

> crate::models::ScalewayRdbV1Snapshot create_snapshot(region, instance_id, inline_object22)
Create an instance snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**inline_object22** | [**InlineObject22**](InlineObject22.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Snapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snapshot

> crate::models::ScalewayRdbV1Snapshot delete_snapshot(region, snapshot_id)
Delete an instance snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot to delete | [required] |

### Return type

[**crate::models::ScalewayRdbV1Snapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshot

> crate::models::ScalewayRdbV1Snapshot get_snapshot(region, snapshot_id)
Get an instance snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot | [required] |

### Return type

[**crate::models::ScalewayRdbV1Snapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_snapshots

> crate::models::ScalewayRdbV1ListSnapshotsResponse list_snapshots(region, name, order_by, instance_id, organization_id, project_id, page, page_size)
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
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayRdbV1ListSnapshotsResponse**](scaleway.rdb.v1.ListSnapshotsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_snapshot

> crate::models::ScalewayRdbV1Snapshot update_snapshot(region, snapshot_id, inline_object26)
Update an instance snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot to update | [required] |
**inline_object26** | [**InlineObject26**](InlineObject26.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Snapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

