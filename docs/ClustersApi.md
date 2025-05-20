# \ClustersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cluster**](ClustersApi.md#create_cluster) | **POST** /k8s/v1/regions/{region}/clusters | Create a new Cluster
[**delete_cluster**](ClustersApi.md#delete_cluster) | **DELETE** /k8s/v1/regions/{region}/clusters/{cluster_id} | Delete a Cluster
[**get_cluster**](ClustersApi.md#get_cluster) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id} | Get a Cluster
[**get_cluster_kube_config**](ClustersApi.md#get_cluster_kube_config) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/kubeconfig | Download the kubeconfig for a Cluster
[**list_cluster_available_types**](ClustersApi.md#list_cluster_available_types) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/available-types | List available cluster types for a cluster
[**list_cluster_available_versions**](ClustersApi.md#list_cluster_available_versions) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/available-versions | List available versions for a Cluster
[**list_clusters**](ClustersApi.md#list_clusters) | **GET** /k8s/v1/regions/{region}/clusters | List Clusters
[**reset_cluster_admin_token**](ClustersApi.md#reset_cluster_admin_token) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/reset-admin-token | Reset the admin token of a Cluster
[**set_cluster_type**](ClustersApi.md#set_cluster_type) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/set-type | Change the Cluster type
[**update_cluster**](ClustersApi.md#update_cluster) | **PATCH** /k8s/v1/regions/{region}/clusters/{cluster_id} | Update a Cluster
[**upgrade_cluster**](ClustersApi.md#upgrade_cluster) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/upgrade | Upgrade a Cluster



## create_cluster

> models::ScalewayPeriodK8sPeriodV1PeriodCluster create_cluster(region, create_cluster_request)
Create a new Cluster

Create a new Kubernetes cluster in a Scaleway region.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**create_cluster_request** | [**CreateClusterRequest**](CreateClusterRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cluster

> models::ScalewayPeriodK8sPeriodV1PeriodCluster delete_cluster(region, cluster_id, with_additional_resources)
Delete a Cluster

Delete a specific Kubernetes cluster and all its associated pools and nodes, and possibly its associated Load Balancers or Block Volumes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | ID of the cluster to delete. | [required] |
**with_additional_resources** | **bool** | Defines whether all volumes (including retain volume type), empty Private Networks and Load Balancers with a name starting with the cluster ID will also be deleted. | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster

> models::ScalewayPeriodK8sPeriodV1PeriodCluster get_cluster(region, cluster_id)
Get a Cluster

Retrieve information about a specific Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | ID of the requested cluster. | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_kube_config

> models::ScalewayPeriodStdPeriodFile get_cluster_kube_config(region, cluster_id, redacted)
Download the kubeconfig for a Cluster

Download the Kubernetes cluster config file (also known as `kubeconfig`) for a specific cluster to use it with `kubectl`. Tip: add `?dl=1` at the end of the URL to directly retrieve the base64 decoded kubeconfig. If you choose not to, the kubeconfig will be base64 encoded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | Cluster ID for which to download the kubeconfig. | [required] |
**redacted** | Option<**bool**> | Hide the legacy token from the kubeconfig. |  |

### Return type

[**models::ScalewayPeriodStdPeriodFile**](scaleway.std.File.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_available_types

> models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableTypesResponse list_cluster_available_types(region, cluster_id)
List available cluster types for a cluster

List the cluster types that a specific Kubernetes cluster is allowed to switch to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | Cluster ID for which the available Kubernetes types will be listed. | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableTypesResponse**](scaleway.k8s.v1.ListClusterAvailableTypesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_available_versions

> models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableVersionsResponse list_cluster_available_versions(region, cluster_id)
List available versions for a Cluster

List the versions that a specific Kubernetes cluster is allowed to upgrade to. Results will include every patch version greater than the current patch, as well as one minor version ahead of the current version. Any upgrade skipping a minor version will not work.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | Cluster ID for which the available Kubernetes versions will be listed. | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableVersionsResponse**](scaleway.k8s.v1.ListClusterAvailableVersionsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_clusters

> models::ScalewayPeriodK8sPeriodV1PeriodListClustersResponse list_clusters(region, organization_id, project_id, order_by, page, page_size, name, status, r#type, private_network_id)
List Clusters

List all existing Kubernetes clusters in a specific region.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**organization_id** | Option<**String**> | Organization ID on which to filter the returned clusters. |  |
**project_id** | Option<**String**> | Project ID on which to filter the returned clusters. |  |
**order_by** | Option<**String**> | Sort order of returned clusters. |  |[default to created_at_asc]
**page** | Option<**i32**> | Page number to return for clusters, from the paginated results. |  |
**page_size** | Option<**i32**> | Maximum number of clusters per page. |  |
**name** | Option<**String**> | Name to filter on, only clusters containing this substring in their name will be returned. |  |
**status** | Option<**String**> | Status to filter on, only clusters with this status will be returned. |  |[default to unknown]
**r#type** | Option<**String**> | Type to filter on, only clusters with this type will be returned. |  |
**private_network_id** | Option<**String**> | Private Network ID to filter on, only clusters within this Private Network will be returned. |  |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodListClustersResponse**](scaleway.k8s.v1.ListClustersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_cluster_admin_token

> reset_cluster_admin_token(region, cluster_id, body)
Reset the admin token of a Cluster

Reset the admin token for a specific Kubernetes cluster. This will revoke the old admin token (which will not be usable afterwards) and create a new one. Note that you will need to download the kubeconfig again to keep interacting with the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | Cluster ID on which the admin token will be renewed. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_cluster_type

> models::ScalewayPeriodK8sPeriodV1PeriodCluster set_cluster_type(region, cluster_id, set_cluster_type_request)
Change the Cluster type

Change the type of a specific Kubernetes cluster. To see the possible values you can enter for the `type` field, [list available cluster types](#list-available-cluster-types-for-a-cluster).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | ID of the cluster to migrate from one type to another. | [required] |
**set_cluster_type_request** | [**SetClusterTypeRequest**](SetClusterTypeRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cluster

> models::ScalewayPeriodK8sPeriodV1PeriodCluster update_cluster(region, cluster_id, update_cluster_request)
Update a Cluster

Update information on a specific Kubernetes cluster. You can update details such as its name, description, tags and configuration. To upgrade a cluster, you will need to use the dedicated endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | ID of the cluster to update. | [required] |
**update_cluster_request** | [**UpdateClusterRequest**](UpdateClusterRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_cluster

> models::ScalewayPeriodK8sPeriodV1PeriodCluster upgrade_cluster(region, cluster_id, upgrade_cluster_request)
Upgrade a Cluster

Upgrade a specific Kubernetes cluster and possibly its associated pools to a specific and supported Kubernetes version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | ID of the cluster to upgrade. | [required] |
**upgrade_cluster_request** | [**UpgradeClusterRequest**](UpgradeClusterRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodCluster**](scaleway.k8s.v1.Cluster.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

