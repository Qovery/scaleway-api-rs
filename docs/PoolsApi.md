# \PoolsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pool**](PoolsApi.md#create_pool) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/pools | Create a new Pool in a Cluster
[**delete_pool**](PoolsApi.md#delete_pool) | **DELETE** /k8s/v1/regions/{region}/pools/{pool_id} | Delete a Pool in a Cluster
[**get_pool**](PoolsApi.md#get_pool) | **GET** /k8s/v1/regions/{region}/pools/{pool_id} | Get a Pool in a Cluster
[**list_pools**](PoolsApi.md#list_pools) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/pools | List Pools in a Cluster
[**migrate_pools_to_new_images**](PoolsApi.md#migrate_pools_to_new_images) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/migrate-pools-to-new-images | Migrate specific pools or all pools of a cluster to new images.
[**update_pool**](PoolsApi.md#update_pool) | **PATCH** /k8s/v1/regions/{region}/pools/{pool_id} | Update a Pool in a Cluster
[**upgrade_pool**](PoolsApi.md#upgrade_pool) | **POST** /k8s/v1/regions/{region}/pools/{pool_id}/upgrade | Upgrade a Pool in a Cluster



## create_pool

> models::ScalewayPeriodK8sPeriodV1PeriodPool create_pool(region, cluster_id, create_pool_request)
Create a new Pool in a Cluster

Create a new pool in a specific Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | Cluster ID to which the pool will be attached. | [required] |
**create_pool_request** | [**CreatePoolRequest**](CreatePoolRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodPool**](scaleway.k8s.v1.Pool.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pool

> models::ScalewayPeriodK8sPeriodV1PeriodPool delete_pool(region, pool_id)
Delete a Pool in a Cluster

Delete a specific pool from a cluster. Note that all the pool's nodes will also be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**pool_id** | **String** | ID of the pool to delete. | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodPool**](scaleway.k8s.v1.Pool.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pool

> models::ScalewayPeriodK8sPeriodV1PeriodPool get_pool(region, pool_id)
Get a Pool in a Cluster

Retrieve details about a specific pool in a Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**pool_id** | **String** | ID of the requested pool. | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodPool**](scaleway.k8s.v1.Pool.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pools

> models::ScalewayPeriodK8sPeriodV1PeriodListPoolsResponse list_pools(region, cluster_id, order_by, page, page_size, name, status)
List Pools in a Cluster

List all the existing pools for a specific Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | ID of the cluster whose pools will be listed. | [required] |
**order_by** | Option<**String**> | Sort order of returned pools. |  |[default to created_at_asc]
**page** | Option<**i32**> | Page number for the returned pools. |  |
**page_size** | Option<**i32**> | Maximum number of pools per page. |  |
**name** | Option<**String**> | Name to filter on, only pools containing this substring in their name will be returned. |  |
**status** | Option<**String**> | Status to filter on, only pools with this status will be returned. |  |[default to unknown]

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodListPoolsResponse**](scaleway.k8s.v1.ListPoolsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrate_pools_to_new_images

> migrate_pools_to_new_images(region, cluster_id, migrate_pools_to_new_images_request)
Migrate specific pools or all pools of a cluster to new images.

If no pool is specified, all pools of the cluster will be migrated to new images.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** |  | [required] |
**migrate_pools_to_new_images_request** | [**MigratePoolsToNewImagesRequest**](MigratePoolsToNewImagesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pool

> models::ScalewayPeriodK8sPeriodV1PeriodPool update_pool(region, pool_id, update_pool_request)
Update a Pool in a Cluster

Update the attributes of a specific pool, such as its desired size, autoscaling settings, and tags. To upgrade a pool, you will need to use the dedicated endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**pool_id** | **String** | ID of the pool to update. | [required] |
**update_pool_request** | [**UpdatePoolRequest**](UpdatePoolRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodPool**](scaleway.k8s.v1.Pool.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_pool

> models::ScalewayPeriodK8sPeriodV1PeriodPool upgrade_pool(region, pool_id, upgrade_pool_request)
Upgrade a Pool in a Cluster

Upgrade the Kubernetes version of a specific pool. Note that it only works if the targeted version matches the cluster's version. This will drain and replace the nodes in that pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**pool_id** | **String** | ID of the pool to upgrade. | [required] |
**upgrade_pool_request** | [**UpgradePoolRequest**](UpgradePoolRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodPool**](scaleway.k8s.v1.Pool.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

