# \ClustersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cluster**](ClustersApi.md#create_cluster) | **POST** /k8s/v1/regions/{region}/clusters | Create a new cluster
[**delete_cluster**](ClustersApi.md#delete_cluster) | **DELETE** /k8s/v1/regions/{region}/clusters/{cluster_id} | Delete a cluster
[**get_cluster**](ClustersApi.md#get_cluster) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id} | Get a cluster
[**get_cluster_kube_config**](ClustersApi.md#get_cluster_kube_config) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/kubeconfig | Download the kubeconfig for a cluster
[**list_cluster_available_versions**](ClustersApi.md#list_cluster_available_versions) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/available-versions | List available versions for a cluster
[**list_clusters**](ClustersApi.md#list_clusters) | **GET** /k8s/v1/regions/{region}/clusters | List all the clusters
[**reset_cluster_admin_token**](ClustersApi.md#reset_cluster_admin_token) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/reset-admin-token | Reset the admin token of a cluster
[**update_cluster**](ClustersApi.md#update_cluster) | **PATCH** /k8s/v1/regions/{region}/clusters/{cluster_id} | Update a cluster
[**upgrade_cluster**](ClustersApi.md#upgrade_cluster) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/upgrade | Upgrade a cluster



## create_cluster

> crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster create_cluster(region, create_cluster_request)
Create a new cluster

This method allows to create a new Kubernetes cluster on an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**create_cluster_request** | [**CreateClusterRequest**](CreateClusterRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cluster

> crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster delete_cluster(region, cluster_id, with_additional_resources)
Delete a cluster

This method allows to delete a specific cluster and all its associated pools and nodes. Note that this method will not delete any Load Balancers or Block Volumes that are associated with the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | The ID of the cluster to delete | [required] |
**with_additional_resources** | Option<**bool**> | Set true if you want to delete all volumes (including retain volume type) and loadbalancers whose name start with cluster ID |  |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster

> crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster get_cluster(region, cluster_id)
Get a cluster

This method allows to get details about a specific Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | The ID of the requested cluster | [required] |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_kube_config

> crate::models::ScalewayPeriodStdPeriodFile get_cluster_kube_config(region, cluster_id)
Download the kubeconfig for a cluster

This method allows to download the Kubernetes cluster config file (AKA kubeconfig) for a specific cluster in order to use it with, for instance, `kubectl`. Tips: add `?dl=1` at the end of the URL to directly get the base64 decoded kubeconfig. If not, the kubeconfig will be base64 encoded. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | The ID of the cluster to download the kubeconfig from | [required] |

### Return type

[**crate::models::ScalewayPeriodStdPeriodFile**](scaleway.std.File.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_available_versions

> crate::models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableVersionsResponse list_cluster_available_versions(region, cluster_id)
List available versions for a cluster

This method allows to list the versions that a specific Kubernetes cluster is allowed to upgrade to. Note that it will be every patch version greater than the actual one as well a one minor version ahead of the actual one. Upgrades skipping a minor version will not work.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | The ID of the cluster which the available Kuberentes versions will be listed from | [required] |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableVersionsResponse**](scaleway.k8s.v1.ListClusterAvailableVersionsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_clusters

> crate::models::ScalewayPeriodK8sPeriodV1PeriodListClustersResponse list_clusters(region, organization_id, project_id, order_by, page, page_size, name, status, r#type)
List all the clusters

This method allows to list all the existing Kubernetes clusters in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**organization_id** | Option<**String**> | The organization ID on which to filter the returned clusters |  |
**project_id** | Option<**String**> | The project ID on which to filter the returned clusters |  |
**order_by** | Option<**String**> | The sort order of the returned clusters |  |[default to created_at_asc]
**page** | Option<**i32**> | The page number for the returned clusters |  |[default to 1]
**page_size** | Option<**i32**> | The maximum number of clusters per page |  |[default to 20]
**name** | Option<**String**> | The name on which to filter the returned clusters |  |
**status** | Option<**String**> | The status on which to filter the returned clusters |  |[default to unknown]
**r#type** | Option<**String**> | The type on which to filter the returned clusters |  |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodListClustersResponse**](scaleway.k8s.v1.ListClustersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_cluster_admin_token

> reset_cluster_admin_token(region, cluster_id, body)
Reset the admin token of a cluster

This method allows to reset the admin token for a specific Kubernetes cluster. This will invalidate the old admin token (which will not be usable after) and create a new one. Note that the redownload of the kubeconfig will be necessary to keep interacting with the cluster (if the old admin token was used).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | The ID of the cluster of which the admin token will be renewed | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cluster

> crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster update_cluster(region, cluster_id, update_cluster_request)
Update a cluster

This method allows to update a specific Kubernetes cluster. Note that this method is not made to upgrade a Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | The ID of the cluster to update | [required] |
**update_cluster_request** | [**UpdateClusterRequest**](UpdateClusterRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_cluster

> crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster upgrade_cluster(region, cluster_id, upgrade_cluster_request)
Upgrade a cluster

This method allows to upgrade a specific Kubernetes cluster and/or its associated pools to a specific and supported Kubernetes version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | The ID of the cluster to upgrade | [required] |
**upgrade_cluster_request** | [**UpgradeClusterRequest**](UpgradeClusterRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

