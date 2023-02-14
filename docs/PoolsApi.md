# \PoolsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pool**](PoolsApi.md#create_pool) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/pools | Create a new pool in a cluster
[**delete_pool**](PoolsApi.md#delete_pool) | **DELETE** /k8s/v1/regions/{region}/pools/{pool_id} | Delete a pool in a cluster
[**get_pool**](PoolsApi.md#get_pool) | **GET** /k8s/v1/regions/{region}/pools/{pool_id} | Get a pool in a cluster
[**list_pools**](PoolsApi.md#list_pools) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/pools | List all the pools in a cluster
[**update_pool**](PoolsApi.md#update_pool) | **PATCH** /k8s/v1/regions/{region}/pools/{pool_id} | Update a pool in a cluster
[**upgrade_pool**](PoolsApi.md#upgrade_pool) | **POST** /k8s/v1/regions/{region}/pools/{pool_id}/upgrade | Upgrade a pool in a cluster



## create_pool

> crate::models::ScalewayPeriodK8sPeriodV1PeriodPool create_pool(region, cluster_id, create_pool_request)
Create a new pool in a cluster

This method allows to create a new pool in a specific Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | The ID of the cluster in which the pool will be created | [required] |
**create_pool_request** | [**CreatePoolRequest**](CreatePoolRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodPool**](scaleway.k8s.v1.Pool.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pool

> crate::models::ScalewayPeriodK8sPeriodV1PeriodPool delete_pool(region, pool_id)
Delete a pool in a cluster

This method allows to delete a specific pool from a cluster, deleting all the nodes associated with it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**pool_id** | **String** | The ID of the pool to delete | [required] |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodPool**](scaleway.k8s.v1.Pool.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pool

> crate::models::ScalewayPeriodK8sPeriodV1PeriodPool get_pool(region, pool_id)
Get a pool in a cluster

This method allows to get details about a specific pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**pool_id** | **String** | The ID of the requested pool | [required] |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodPool**](scaleway.k8s.v1.Pool.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pools

> crate::models::ScalewayPeriodK8sPeriodV1PeriodListPoolsResponse list_pools(region, cluster_id, order_by, page, page_size, name, status)
List all the pools in a cluster

This method allows to list all the existing pools for a specific Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | The ID of the cluster from which the pools will be listed from | [required] |
**order_by** | Option<**String**> | The sort order of the returned pools |  |[default to created_at_asc]
**page** | Option<**i32**> | The page number for the returned pools |  |[default to 1]
**page_size** | Option<**i32**> | The maximum number of pools per page |  |[default to 20]
**name** | Option<**String**> | The name on which to filter the returned pools |  |
**status** | Option<**String**> | The status on which to filter the returned pools |  |[default to unknown]

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodListPoolsResponse**](scaleway.k8s.v1.ListPoolsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pool

> crate::models::ScalewayPeriodK8sPeriodV1PeriodPool update_pool(region, pool_id, update_pool_request)
Update a pool in a cluster

This method allows to update some attributes of a specific pool such as the size, the autoscaling enablement, the tags, ...

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**pool_id** | **String** | The ID of the pool to update | [required] |
**update_pool_request** | [**UpdatePoolRequest**](UpdatePoolRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodPool**](scaleway.k8s.v1.Pool.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_pool

> crate::models::ScalewayPeriodK8sPeriodV1PeriodPool upgrade_pool(region, pool_id, upgrade_pool_request)
Upgrade a pool in a cluster

This method allows to upgrade the Kubernetes version of a specific pool. Note that this will work when the targeted version is the same than the version of the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**pool_id** | **String** | The ID of the pool to upgrade | [required] |
**upgrade_pool_request** | [**UpgradePoolRequest**](UpgradePoolRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodPool**](scaleway.k8s.v1.Pool.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

