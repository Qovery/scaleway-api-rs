# \NodesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_external_node**](NodesApi.md#create_external_node) | **POST** /k8s/v1/regions/{region}/pools/{pool_id}/external-nodes | Create a Kosmos node
[**delete_node**](NodesApi.md#delete_node) | **DELETE** /k8s/v1/regions/{region}/nodes/{node_id} | Delete a Node in a Cluster
[**get_node**](NodesApi.md#get_node) | **GET** /k8s/v1/regions/{region}/nodes/{node_id} | Get a Node in a Cluster
[**list_nodes**](NodesApi.md#list_nodes) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/nodes | List Nodes in a Cluster
[**reboot_node**](NodesApi.md#reboot_node) | **POST** /k8s/v1/regions/{region}/nodes/{node_id}/reboot | Reboot a Node in a Cluster



## create_external_node

> models::ScalewayPeriodK8sPeriodV1PeriodExternalNode create_external_node(region, pool_id, body)
Create a Kosmos node

Retrieve metadata for a Kosmos node. This method is not intended to be called by end users but rather programmatically by the kapsule-node-agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**pool_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodExternalNode**](scaleway.k8s.v1.ExternalNode.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_node

> models::ScalewayPeriodK8sPeriodV1PeriodNode delete_node(region, node_id, skip_drain, replace)
Delete a Node in a Cluster

Delete a specific Node. The node will first be drained and pods will be rescheduled onto another node. Note that when there is not enough space to reschedule all the pods (such as in a one-node cluster, or with specific constraints), disruption of your applications may occur.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**node_id** | **String** | ID of the node to replace. | [required] |
**skip_drain** | **bool** | Skip draining node from its workload (Note: this parameter is currently inactive). | [required] |
**replace** | **bool** | Add a new node after the deletion of this node. | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodNode**](scaleway.k8s.v1.Node.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node

> models::ScalewayPeriodK8sPeriodV1PeriodNode get_node(region, node_id)
Get a Node in a Cluster

Retrieve details about a specific Kubernetes Node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**node_id** | **String** | ID of the requested node. | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodNode**](scaleway.k8s.v1.Node.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nodes

> models::ScalewayPeriodK8sPeriodV1PeriodListNodesResponse list_nodes(region, cluster_id, pool_id, order_by, page, page_size, name, status)
List Nodes in a Cluster

List all the existing nodes for a specific Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | Cluster ID from which the nodes will be listed from. | [required] |
**pool_id** | Option<**String**> | Pool ID on which to filter the returned nodes. |  |
**order_by** | Option<**String**> | Sort order of the returned nodes. |  |[default to created_at_asc]
**page** | Option<**i32**> | Page number for the returned nodes. |  |
**page_size** | Option<**i32**> | Maximum number of nodes per page. |  |
**name** | Option<**String**> | Name to filter on, only nodes containing this substring in their name will be returned. |  |
**status** | Option<**String**> | Status to filter on, only nodes with this status will be returned. |  |[default to unknown]

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodListNodesResponse**](scaleway.k8s.v1.ListNodesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reboot_node

> models::ScalewayPeriodK8sPeriodV1PeriodNode reboot_node(region, node_id, body)
Reboot a Node in a Cluster

Reboot a specific Node. The node will first be drained and pods will be rescheduled onto another node. Note that when there is not enough space to reschedule all the pods (such as in a one-node cluster, or with specific constraints), disruption of your applications may occur.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**node_id** | **String** | ID of the node to reboot. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodNode**](scaleway.k8s.v1.Node.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

