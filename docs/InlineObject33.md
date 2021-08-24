# InlineObject33

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the pool | 
**node_type** | **String** | The node type is the type of Scaleway Instance wanted for the pool | 
**placement_group_id** | Option<**String**> | The placement group ID in which all the nodes of the pool will be created | [optional]
**autoscaling** | Option<**bool**> | The enablement of the autoscaling feature for the pool | [optional]
**size** | **f32** | The size (number of nodes) of the pool | 
**min_size** | Option<**f32**> | The minimun size of the pool. Note that this fields will be used only when autoscaling is enabled. | [optional]
**max_size** | Option<**f32**> | The maximum size of the pool. Note that this fields will be used only when autoscaling is enabled. | [optional]
**container_runtime** | Option<**String**> | The customization of the container runtime is available for each pool. Note that `docker` is the only supporter runtime at the moment. Others are to be considered experimental.  | [optional][default to ContainerRuntime_UnknownRuntime]
**autohealing** | Option<**bool**> | The enablement of the autohealing feature for the pool | [optional]
**tags** | Option<**Vec<String>**> | The tags associated with the pool | [optional]
**kubelet_args** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The Kubelet arguments to be used by this pool. Note that this feature is to be considered as experimental | [optional]
**upgrade_policy** | Option<[**crate::models::K8sV1RegionsRegionClustersClusterIdPoolsUpgradePolicy**](_k8s_v1_regions__region__clusters__cluster_id__pools_upgrade_policy.md)> |  | [optional]
**zone** | Option<**String**> | The Zone in which the Pool's node will be spawn in | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


