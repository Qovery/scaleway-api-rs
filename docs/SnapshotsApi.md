# \SnapshotsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_instance_from_snapshot**](SnapshotsApi.md#create_instance_from_snapshot) | **post** /rdb/v1/regions/{region}/snapshots/{snapshot_id}/create-instance | Create a new instance from a given snapshot
[**create_snapshot**](SnapshotsApi.md#create_snapshot) | **post** /instance/v1/zones/{zone}/snapshots | Create a snapshot from a given volume
[**create_snapshot1**](SnapshotsApi.md#create_snapshot1) | **post** /rdb/v1/regions/{region}/instances/{instance_id}/snapshots | Create an instance snapshot
[**delete_snapshot**](SnapshotsApi.md#delete_snapshot) | **delete** /instance/v1/zones/{zone}/snapshots/{snapshot_id} | Delete a snapshot
[**delete_snapshot1**](SnapshotsApi.md#delete_snapshot1) | **delete** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Delete an instance snapshot
[**get_snapshot**](SnapshotsApi.md#get_snapshot) | **get** /instance/v1/zones/{zone}/snapshots/{snapshot_id} | Get a snapshot
[**get_snapshot1**](SnapshotsApi.md#get_snapshot1) | **get** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Get an instance snapshot
[**list_snapshots**](SnapshotsApi.md#list_snapshots) | **get** /instance/v1/zones/{zone}/snapshots | List snapshots
[**list_snapshots1**](SnapshotsApi.md#list_snapshots1) | **get** /rdb/v1/regions/{region}/snapshots | List instance snapshots
[**set_snapshot**](SnapshotsApi.md#set_snapshot) | **put** /instance/v1/zones/{zone}/snapshots/{snapshot_id} | Update snapshot
[**update_snapshot**](SnapshotsApi.md#update_snapshot) | **patch** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Update an instance snapshot



## create_instance_from_snapshot

> crate::models::ScalewayRdbV1Instance create_instance_from_snapshot(region, snapshot_id, inline_object75)
Create a new instance from a given snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | Block snapshot of the instance | [required] |
**inline_object75** | [**InlineObject75**](InlineObject75.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Instance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_snapshot

> crate::models::ScalewayInstanceV1CreateSnapshotResponse create_snapshot(zone, inline_object17)
Create a snapshot from a given volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object17** | [**InlineObject17**](InlineObject17.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1CreateSnapshotResponse**](scaleway.instance.v1.CreateSnapshotResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_snapshot1

> crate::models::ScalewayRdbV1Snapshot create_snapshot1(region, instance_id, inline_object70)
Create an instance snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**inline_object70** | [**InlineObject70**](InlineObject70.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Snapshot**](scaleway.rdb.v1.Snapshot.md)

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

> crate::models::ScalewayRdbV1Snapshot delete_snapshot1(region, snapshot_id)
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

> crate::models::ScalewayInstanceV1GetSnapshotResponse get_snapshot(zone, snapshot_id)
Get a snapshot

Get details of a snapshot with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot you want to get | [required] |

### Return type

[**crate::models::ScalewayInstanceV1GetSnapshotResponse**](scaleway.instance.v1.GetSnapshotResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshot1

> crate::models::ScalewayRdbV1Snapshot get_snapshot1(region, snapshot_id)
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

> crate::models::ScalewayInstanceV1ListSnapshotsResponse list_snapshots(zone, organization, per_page, page, name, project)
List snapshots

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**organization** | Option<**String**> |  |  |
**per_page** | Option<**f32**> |  |  |
**page** | Option<**f32**> | Page number |  |[default to 1]
**name** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |

### Return type

[**crate::models::ScalewayInstanceV1ListSnapshotsResponse**](scaleway.instance.v1.ListSnapshotsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_snapshots1

> crate::models::ScalewayRdbV1ListSnapshotsResponse list_snapshots1(region, name, order_by, instance_id, organization_id, project_id, page, page_size)
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


## set_snapshot

> crate::models::ScalewayInstanceV1SetSnapshotResponse set_snapshot(zone, snapshot_id, inline_object18)
Update snapshot

Replace all snapshot properties with a snapshot message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**snapshot_id** | **String** |  | [required] |
**inline_object18** | [**InlineObject18**](InlineObject18.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1SetSnapshotResponse**](scaleway.instance.v1.SetSnapshotResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_snapshot

> crate::models::ScalewayRdbV1Snapshot update_snapshot(region, snapshot_id, inline_object74)
Update an instance snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**snapshot_id** | **String** | UUID of the snapshot to update | [required] |
**inline_object74** | [**InlineObject74**](InlineObject74.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1Snapshot**](scaleway.rdb.v1.Snapshot.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

