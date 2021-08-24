# \NodesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_node**](NodesApi.md#get_node) | **GET** /k8s/v1/regions/{region}/nodes/{node_id} | Get a node in a cluster
[**list_nodes**](NodesApi.md#list_nodes) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/nodes | List all the nodes in a cluster
[**reboot_node**](NodesApi.md#reboot_node) | **POST** /k8s/v1/regions/{region}/nodes/{node_id}/reboot | Reboot a node in a cluster
[**replace_node**](NodesApi.md#replace_node) | **POST** /k8s/v1/regions/{region}/nodes/{node_id}/replace | Replace a node in a cluster



## get_node

> crate::models::ScalewayK8sV1Node get_node(region, node_id)
Get a node in a cluster

This method allows to get details about a specific Kubernetes node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**node_id** | **String** | The ID of the requested node | [required] |

### Return type

[**crate::models::ScalewayK8sV1Node**](scaleway.k8s.v1.Node.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nodes

> crate::models::ScalewayK8sV1ListNodesResponse list_nodes(region, cluster_id, pool_id, order_by, page, page_size, name, status)
List all the nodes in a cluster

This method allows to list all the existing nodes for a specific Kubernetes cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | The cluster ID from which the nodes will be listed from | [required] |
**pool_id** | Option<**String**> | The pool ID on which to filter the returned nodes |  |
**order_by** | Option<**String**> | The sort order of the returned nodes |  |[default to created_at_asc]
**page** | Option<**f32**> | The page number for the returned nodes |  |[default to 1]
**page_size** | Option<**f32**> | The maximum number of nodes per page |  |[default to 20]
**name** | Option<**String**> | The name on which to filter the returned nodes |  |
**status** | Option<**String**> | The status on which to filter the returned nodes |  |[default to unknown]

### Return type

[**crate::models::ScalewayK8sV1ListNodesResponse**](scaleway.k8s.v1.ListNodesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reboot_node

> crate::models::ScalewayK8sV1Node reboot_node(region, node_id, body)
Reboot a node in a cluster

This method allows to reboot a specific node. This node will frist be cordoned, meaning that scheduling will be disabled. Then the existing pods on the node will be drained and reschedule onto another schedulable node. Note that when there is not enough space to reschedule all the pods (in a one node cluster for instance), you may experience some disruption of your applications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**node_id** | **String** | The ID of the node to reboot | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayK8sV1Node**](scaleway.k8s.v1.Node.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_node

> crate::models::ScalewayK8sV1Node replace_node(region, node_id, body)
Replace a node in a cluster

This method allows to replace a specific node. The node will be set cordoned, meaning that scheduling will be disabled. Then the existing pods on the node will be drained and reschedule onto another schedulable node. Then the node will be deleted, and a new one will be created after the deletion. Note that when there is not enough space to reschedule all the pods (in a one node cluster for instance), you may experience some disruption of your applications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**node_id** | **String** | The ID of the node to replace | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayK8sV1Node**](scaleway.k8s.v1.Node.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

