# \SnapshotsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_snapshot**](SnapshotsApi.md#create_snapshot) | **post** /instance/v1/zones/{zone}/snapshots | Create a snapshot from a given volume
[**delete_snapshot**](SnapshotsApi.md#delete_snapshot) | **delete** /instance/v1/zones/{zone}/snapshots/{snapshot_id} | Delete a snapshot
[**get_snapshot**](SnapshotsApi.md#get_snapshot) | **get** /instance/v1/zones/{zone}/snapshots/{snapshot_id} | Get a snapshot
[**list_snapshots**](SnapshotsApi.md#list_snapshots) | **get** /instance/v1/zones/{zone}/snapshots | List snapshots
[**set_snapshot**](SnapshotsApi.md#set_snapshot) | **put** /instance/v1/zones/{zone}/snapshots/{snapshot_id} | Update snapshot



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

