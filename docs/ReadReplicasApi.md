# \ReadReplicasApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_read_replica**](ReadReplicasApi.md#create_read_replica) | **POST** /rdb/v1/regions/{region}/read-replicas | Create a read replica
[**create_read_replica_endpoint**](ReadReplicasApi.md#create_read_replica_endpoint) | **POST** /rdb/v1/regions/{region}/read-replicas/{read_replica_id}/endpoints | Create a new endpoint for a given read replica
[**delete_read_replica**](ReadReplicasApi.md#delete_read_replica) | **DELETE** /rdb/v1/regions/{region}/read-replicas/{read_replica_id} | Delete a read replica
[**get_read_replica**](ReadReplicasApi.md#get_read_replica) | **GET** /rdb/v1/regions/{region}/read-replicas/{read_replica_id} | Get a read replica
[**reset_read_replica**](ReadReplicasApi.md#reset_read_replica) | **POST** /rdb/v1/regions/{region}/read-replicas/{read_replica_id}/reset | Resync a read replica



## create_read_replica

> crate::models::ScalewayPeriodRdbPeriodV1PeriodReadReplica create_read_replica(region, create_read_replica_request)
Create a read replica

You can only create a maximum of 3 read replicas for one instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**create_read_replica_request** | [**CreateReadReplicaRequest**](CreateReadReplicaRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodReadReplica**](scaleway.rdb.v1.ReadReplica.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_read_replica_endpoint

> crate::models::ScalewayPeriodRdbPeriodV1PeriodReadReplica create_read_replica_endpoint(region, read_replica_id, create_read_replica_endpoint_request)
Create a new endpoint for a given read replica

A read replica can have at most one direct access and one private network endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**read_replica_id** | **String** | UUID of the read replica (UUID format) | [required] |
**create_read_replica_endpoint_request** | [**CreateReadReplicaEndpointRequest**](CreateReadReplicaEndpointRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodReadReplica**](scaleway.rdb.v1.ReadReplica.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_read_replica

> crate::models::ScalewayPeriodRdbPeriodV1PeriodReadReplica delete_read_replica(region, read_replica_id)
Delete a read replica

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**read_replica_id** | **String** | UUID of the read replica (UUID format) | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodReadReplica**](scaleway.rdb.v1.ReadReplica.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_read_replica

> crate::models::ScalewayPeriodRdbPeriodV1PeriodReadReplica get_read_replica(region, read_replica_id)
Get a read replica

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**read_replica_id** | **String** | UUID of the read replica (UUID format) | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodReadReplica**](scaleway.rdb.v1.ReadReplica.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_read_replica

> crate::models::ScalewayPeriodRdbPeriodV1PeriodReadReplica reset_read_replica(region, read_replica_id, body)
Resync a read replica

When you resync a read replica, first it is reset, and then its data is resynchronized from the primary node. Your read replica will be unavailable during the resync process. The duration of this process is proportional to your Database Instance size. The configured endpoints will not change. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**read_replica_id** | **String** | UUID of the read replica (UUID format) | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodReadReplica**](scaleway.rdb.v1.ReadReplica.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

