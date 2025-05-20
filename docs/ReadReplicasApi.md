# \ReadReplicasApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_read_replica**](ReadReplicasApi.md#create_read_replica) | **POST** /rdb/v1/regions/{region}/read-replicas | Create a Read Replica
[**create_read_replica_endpoint**](ReadReplicasApi.md#create_read_replica_endpoint) | **POST** /rdb/v1/regions/{region}/read-replicas/{read_replica_id}/endpoints | Create an endpoint for a Read Replica
[**delete_read_replica**](ReadReplicasApi.md#delete_read_replica) | **DELETE** /rdb/v1/regions/{region}/read-replicas/{read_replica_id} | Delete a Read Replica
[**get_read_replica**](ReadReplicasApi.md#get_read_replica) | **GET** /rdb/v1/regions/{region}/read-replicas/{read_replica_id} | Get a Read Replica
[**promote_read_replica**](ReadReplicasApi.md#promote_read_replica) | **POST** /rdb/v1/regions/{region}/read-replicas/{read_replica_id}/promote | Promote a Read Replica
[**reset_read_replica**](ReadReplicasApi.md#reset_read_replica) | **POST** /rdb/v1/regions/{region}/read-replicas/{read_replica_id}/reset | Resync a Read Replica



## create_read_replica

> models::ScalewayPeriodRdbPeriodV1PeriodReadReplica create_read_replica(region, create_read_replica_request)
Create a Read Replica

Create a new Read Replica of a Database Instance. You must specify the `region` and the `instance_id`. You can only create a maximum of 3 Read Replicas per Database Instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**create_read_replica_request** | [**CreateReadReplicaRequest**](CreateReadReplicaRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodReadReplica**](scaleway.rdb.v1.ReadReplica.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_read_replica_endpoint

> models::ScalewayPeriodRdbPeriodV1PeriodReadReplica create_read_replica_endpoint(region, read_replica_id, create_read_replica_endpoint_request)
Create an endpoint for a Read Replica

Create a new endpoint for a Read Replica. Read Replicas can have at most one direct access and one Private Network endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**read_replica_id** | **String** | UUID of the Read Replica. (UUID format) | [required] |
**create_read_replica_endpoint_request** | [**CreateReadReplicaEndpointRequest**](CreateReadReplicaEndpointRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodReadReplica**](scaleway.rdb.v1.ReadReplica.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_read_replica

> models::ScalewayPeriodRdbPeriodV1PeriodReadReplica delete_read_replica(region, read_replica_id)
Delete a Read Replica

Delete a Read Replica of a Database Instance. You must specify the `region` and `read_replica_id` parameters of the Read Replica you want to delete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**read_replica_id** | **String** | UUID of the Read Replica. (UUID format) | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodReadReplica**](scaleway.rdb.v1.ReadReplica.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_read_replica

> models::ScalewayPeriodRdbPeriodV1PeriodReadReplica get_read_replica(region, read_replica_id)
Get a Read Replica

Retrieve information about a Database Instance Read Replica. Full details about the Read Replica, like `endpoints`, `status`  and `region` are returned in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**read_replica_id** | **String** | UUID of the Read Replica. (UUID format) | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodReadReplica**](scaleway.rdb.v1.ReadReplica.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promote_read_replica

> models::ScalewayPeriodRdbPeriodV1PeriodInstance promote_read_replica(region, read_replica_id, body)
Promote a Read Replica

Promote a Read Replica to Database Instance automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**read_replica_id** | **String** | UUID of the Read Replica. (UUID format) | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodInstance**](scaleway.rdb.v1.Instance.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_read_replica

> models::ScalewayPeriodRdbPeriodV1PeriodReadReplica reset_read_replica(region, read_replica_id, body)
Resync a Read Replica

When you resync a Read Replica, first it is reset, then its data is resynchronized from the primary node. Your Read Replica remains unavailable during the resync process. The duration of this process is proportional to the size of your Database Instance. The configured endpoints do not change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**read_replica_id** | **String** | UUID of the Read Replica. (UUID format) | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodReadReplica**](scaleway.rdb.v1.ReadReplica.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

